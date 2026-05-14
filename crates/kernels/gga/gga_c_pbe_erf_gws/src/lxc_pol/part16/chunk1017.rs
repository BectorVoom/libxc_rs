//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1017/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1017<F: Float>(t14698: F, t3972: F, t13780: F, t3060: F, t3990: F, t13859: F, t1146: F, t3955: F, t13793: F, t14657: F, t13808: F, t4138: F, t14423: F, t875: F, t13796: F, t3989: F) -> (F, F, F, F, F, F, F, F) {
    let t14699 = t3972 * t14698;
    let t14705 = t3990 * t13780 * t3060;
    let t14706 = t13859 * t14705;
    let t14708 = t3955 * t1146;
    let t14714 = t14657 * t13793;
    let t14716 = t13808 * t4138;
    let t14720 = t14423 * t875;
    let t14721 = t13796 * t14720;
    let t14722 = t3989 * t14721;
    (t14699, t14705, t14706, t14708, t14714, t14716, t14721, t14722)
}
