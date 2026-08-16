//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 909/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk909(t18552: f64, t449: f64, t456: f64, t470: f64, t1272: f64, t1289: f64, t13: f64, t18515: f64, t4661: f64, t1314: f64, t1215: f64, t174: f64, t60: f64) -> (f64, f64, f64, f64, f64) {
    let t18556 = 0.58482233974552040708e0_f64 * t470 * t449 * t18552 * t456;
    let t18562 = 0.620700176468474021e4_f64 * t13 / t1289 / t1272 * t18515 * t4661;
    let t18563 = t1314 * t1314;
    let t18567 = 0.35089340384731224426e1_f64 * t470 * t1215 * t18563 * t456;
    let t18568 = t60 * t174;
    (t18556, t18562, t18563, t18567, t18568)
}
