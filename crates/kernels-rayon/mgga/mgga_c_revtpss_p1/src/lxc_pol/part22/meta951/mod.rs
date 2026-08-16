//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta951 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3193;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3194;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta951(t11262: f64, t3711: f64, t5278: f64, t12640: f64, t1811: f64, t17807: f64, t473: f64, t3766: f64, t5216: f64, t13141: f64, t1770: f64, t1284: f64, t17331: f64, t13126: f64, t1269: f64, t460: f64, t12050: f64, t17710: f64, t17191: f64, t3555: f64, t13147: f64, t1209: f64, t21455: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t59426, t59464, t59488, t59492, t59498, t59537) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3193(t11262, t3711, t5278, t12640, t1811, t17807, t473, t3766, t5216, t13141, t1770, t1284, t17331);
        let (t59550, t59591, t59650, t59657, t59671, t59674) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3194(t13126, t1770, t1269, t13141, t460, t12050, t17710, t17191, t3555, t13147, t1209, t21455);
    (t59426, t59464, t59488, t59492, t59498, t59537, t59550, t59591, t59650, t59657, t59671, t59674)
}
