//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 909/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk909<F: Float>(t18552: F, t449: F, t456: F, t470: F, t1272: F, t1289: F, t13: F, t18515: F, t4661: F, t1314: F, t1215: F, t174: F, t60: F) -> (F, F, F, F, F) {
    let t18556 = F::cast_from(0.58482233974552040708e0_f64) * t470 * t449 * t18552 * t456;
    let t18562 = F::cast_from(0.620700176468474021e4_f64) * t13 / t1289 / t1272 * t18515 * t4661;
    let t18563 = t1314 * t1314;
    let t18567 = F::cast_from(0.35089340384731224426e1_f64) * t470 * t1215 * t18563 * t456;
    let t18568 = t60 * t174;
    (t18556, t18562, t18563, t18567, t18568)
}
