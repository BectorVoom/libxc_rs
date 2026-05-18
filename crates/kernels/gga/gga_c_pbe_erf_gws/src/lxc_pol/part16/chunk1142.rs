//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1142/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1142<F: Float>(t2306: F, t820: F, t9385: F, t3975: F, t3972: F, t13780: F, t3060: F, t3990: F, t13859: F, t1146: F, t3955: F, t13793: F, t14657: F) -> (F, F, F, F, F, F, F) {
    let t14696 = t2306 * t820;
    let t14697 = t9385 * t14696;
    let t14698 = t3975 * t14697;
    let t14699 = t3972 * t14698;
    let t14705 = t3990 * t13780 * t3060;
    let t14706 = t13859 * t14705;
    let t14708 = t3955 * t1146;
    let t14714 = t14657 * t13793;
    (t14696, t14698, t14699, t14705, t14706, t14708, t14714)
}
