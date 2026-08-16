//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta491 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1747;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1748;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1749;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1750;
use chunk4::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1751;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta491(t27213: f64, t7407: f64, t1956: f64, t26508: f64, t26521: f64, t26522: f64, t26529: f64, t26534: f64, t26536: f64, t26538: f64, t27199: f64, t28400: f64, t28405: f64, t28411: f64, t28418: f64, t4487: f64, t7070: f64, t7403: f64, t7420: f64, t2061: f64, t2718: f64, t14587: f64, t26497: f64, t4481: f64, t26550: f64, t27349: f64, t14495: f64, t27312: f64, t212: f64, t7997: f64, t780: f64, t689: f64, t2067: f64, t25391: f64, t26541: f64, t26545: f64, t26557: f64, t26558: f64, t26561: f64, t26564: f64, t26578: f64, t27275: f64, t27353: f64, t7415: f64, t28358: f64, t28397: f64, t892: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t28422, t28424) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1747(t27213, t7407, t1956, t26508, t26521, t26522, t26529, t26534, t26536, t26538, t27199, t28400, t28405, t28411, t28418, t4487, t7070, t7403, t7420);
        let t28425 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1748(t2061, t2718);
        let (t28426, t28434, t28436, t28439, t28442, t28447, t28448, t28449) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1749(t14587, t28425, t26497, t4481, t26550, t27349, t14495, t27312, t212, t7997, t780, t689);
        let t28453 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1750(t2067, t25391, t26541, t26545, t26557, t26558, t26561, t26564, t26578, t27199, t27275, t27353, t28426, t28434, t28436, t28439, t28442, t28449, t7415);
        let (t28455, t28456) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1751(t28358, t28397, t28424, t28453, t892);
    (t28422, t28425, t28426, t28434, t28436, t28439, t28442, t28447, t28448, t28449, t28455, t28456)
}
