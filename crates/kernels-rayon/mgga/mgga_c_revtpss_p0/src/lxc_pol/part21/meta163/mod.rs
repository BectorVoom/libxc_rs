//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta163 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1038;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1039;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1040;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1041;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1042;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1043;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1044;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta163(t1312: f64, t2320: f64, t2322: f64, t2327: f64, t2371: f64, t670: f64, t93: f64, t1330: f64, t72: f64, t757: f64, t530: f64, t566: f64, t1353: f64, t525: f64, t605: f64, t30: f64, t2257: f64, t513: f64, t527: f64, zeta_threshold: f64, t1113: f64, t33: f64, t3351: f64, t516: f64, t162: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3821, t3825, t3826, t3827, t3828) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1038(t1312, t2320, t2322, t2327, t2371, t670, t93, t1330, t72, t757, t530, t566);
        let t3829 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1039(t1353);
        let t3833 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1040(t525);
        let t3834 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1041(t605);
        let (t3840, t3841) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1042(t30, t2257, t3833, t3834, t513, t527, zeta_threshold);
        let t3842 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1043(t1113);
        let t3850 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1044(t33, t3351, t3841, t3842, t516, t162, t3840, zeta_threshold);
    (t3821, t3825, t3826, t3827, t3828, t3829, t3833, t3834, t3841, t3842, t3850)
}
