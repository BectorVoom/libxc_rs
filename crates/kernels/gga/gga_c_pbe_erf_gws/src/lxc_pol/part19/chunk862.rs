//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 862/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk862<F: Float>(t10681: F, t7062: F, t1651: F, t3503: F, t587: F, t2609: F, t7527: F, t3562: F, t649: F, t661: F, t1621: F, t1620: F, t2627: F, t7130: F, t1010: F, t7793: F) -> (F, F, F, F, F, F) {
    let t10683 = 16.0 / 45.0 * t7062 * t10681;
    let t10685 = t1651 * t3503;
    let t10686 = t587 * t10685;
    let t10687 = 16.0 / 135.0 * t10686;
    let t10690 = 8.0 / 15.0 * t7527 * t2609;
    let t10691 = t649 * t3562;
    let t10692 = t10691 * t661;
    let t10693 = t1621 * t10692;
    let t10695 = 4.0 / 15.0 * t1620 * t10693;
    let t10697 = 8.0 / 15.0 * t7130 * t2627;
    let t10699 = 8.0 / 45.0 * t7793 * t1010;
    (t10683, t10687, t10690, t10695, t10697, t10699)
}
