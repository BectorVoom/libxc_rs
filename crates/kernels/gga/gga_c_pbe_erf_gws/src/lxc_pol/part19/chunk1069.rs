//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1069/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1069<F: Float>(t11731: F, t5: F, t343: F, t2494: F, t3178: F, t3717: F, t3802: F, t6469: F, t11806: F, t810: F, t11661: F, t4395: F, t11583: F, t3703: F, t2079: F, t3780: F) -> (F, F, F, F, F, F, F, F, F) {
    let t37440 = t5 * t11731;
    let t37441 = t37440 * t343;
    let t37454 = t3178 * t2494;
    let t37632 = t3717 * param_a_c;
    let t38036 = t6469 * t3802;
    let t38133 = t11806 * t810;
    let t38537 = t4395 * t11661;
    let t38545 = t11583 * t810;
    let t39052 = t3703 * param_a_c;
    let t39061 = t2079 * t3780;
    (t37441, t37454, t37632, t38036, t38133, t38537, t38545, t39052, t39061)
}
