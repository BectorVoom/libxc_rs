//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 745/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk745<F: Float>(t3733: F, t8662: F, t3893: F, t840: F, t1105: F, t2053: F, t4558: F, t4559: F, t4561: F) -> (F, F, F, F) {
    let t12246 = t8662 * t3733;
    let t12253 = t840 * t3893;
    let t12275 = t2053 * t1105;
    let t12323 = t4558 + t4559 + t4561;
    (t12246, t12253, t12275, t12323)
}
