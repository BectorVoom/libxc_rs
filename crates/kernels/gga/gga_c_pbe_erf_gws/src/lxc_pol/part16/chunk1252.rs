//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1252/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1252<F: Float>(t4127: F, t4419: F, t1133: F, t2416: F, t13796: F, t2417: F, t343: F, t3989: F, t1123: F, t51989: F, t833: F, t850: F) -> (F, F, F) {
    let t53715 = t4127 * t4419;
    let t53717 = t2416 * t1133;
    let t53721 = t3989 * t13796 * t53717 * t343 * t2417;
    let t53725 = t850 * t1123 * t51989 * t833;
    (t53715, t53721, t53725)
}
