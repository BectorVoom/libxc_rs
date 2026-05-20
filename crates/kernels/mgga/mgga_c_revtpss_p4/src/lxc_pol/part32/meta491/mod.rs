//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta491 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1747;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1748;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1749;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1750;
use chunk4::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1751;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta491<F: Float>(t27213: F, t7407: F, t1956: F, t26508: F, t26521: F, t26522: F, t26529: F, t26534: F, t26536: F, t26538: F, t27199: F, t28400: F, t28405: F, t28411: F, t28418: F, t4487: F, t7070: F, t7403: F, t7420: F, t2061: F, t2718: F, t14587: F, t26497: F, t4481: F, t26550: F, t27349: F, t14495: F, t27312: F, t212: F, t7997: F, t780: F, t689: F, t2067: F, t25391: F, t26541: F, t26545: F, t26557: F, t26558: F, t26561: F, t26564: F, t26578: F, t27275: F, t27353: F, t7415: F, t28358: F, t28397: F, t892: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t28422, t28424) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1747::<F>(t27213, t7407, t1956, t26508, t26521, t26522, t26529, t26534, t26536, t26538, t27199, t28400, t28405, t28411, t28418, t4487, t7070, t7403, t7420);
        let t28425 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1748::<F>(t2061, t2718);
        let (t28426, t28434, t28436, t28439, t28442, t28447, t28448, t28449) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1749::<F>(t14587, t28425, t26497, t4481, t26550, t27349, t14495, t27312, t212, t7997, t780, t689);
        let t28453 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1750::<F>(t2067, t25391, t26541, t26545, t26557, t26558, t26561, t26564, t26578, t27199, t27275, t27353, t28426, t28434, t28436, t28439, t28442, t28449, t7415);
        let (t28455, t28456) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1751::<F>(t28358, t28397, t28424, t28453, t892);
    (t28422, t28425, t28426, t28434, t28436, t28439, t28442, t28447, t28448, t28449, t28455, t28456)
}
