//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1207/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1207<F: Float>(t11661: F, t4395: F, t11583: F, t810: F, t3703: F, t2079: F, t3780: F, t3306: F, t8589: F, t2395: F, t19894: F, t3912: F) -> (F, F, F, F, F, F, F) {
    let t38537 = t4395 * t11661;
    let t38545 = t11583 * t810;
    let t39052 = t3703 * param_a_c;
    let t39061 = t2079 * t3780;
    let t39460 = t8589 * t3306;
    let t39579 = t2395 * t3703;
    let t39689 = t3912 * t19894;
    (t38537, t38545, t39052, t39061, t39460, t39579, t39689)
}
