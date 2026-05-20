//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2095/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2095<F: Float>(t13821: F, t27940: F, t13928: F, t26028: F, t241: F, t820: F, t94491: F, t13807: F, t13817: F, t13991: F, t13793: F, t13786: F) -> (F, F, F, F, F, F, F) {
    let t98110 = t27940 * t13821;
    let t98112 = t26028 * t13928;
    let t98115 = t820 * t94491 * t241;
    let t98116 = t98115 * t13807;
    let t98118 = t27940 * t13817;
    let t98120 = t27940 * t13991;
    let t98122 = t27940 * t13793;
    let t98124 = t26028 * t13786;
    (t98110, t98112, t98116, t98118, t98120, t98122, t98124)
}
