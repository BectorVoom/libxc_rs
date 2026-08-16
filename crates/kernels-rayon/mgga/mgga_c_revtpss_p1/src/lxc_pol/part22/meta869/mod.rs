//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta869 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3027;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3028;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta869(t14923: f64, t14927: f64, t10811: f64, t14697: f64, t40672: f64, t828: f64, t10905: f64, t14825: f64, t14829: f64, t14819: f64, t40517: f64, t14910: f64, t4423: f64, t836: f64, t14741: f64, t2710: f64, t2713: f64, t10744: f64, t14861: f64, t808: f64, t40791: f64, t4442: f64, t14468: f64, t236: f64, t807: f64, t854: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t51000, t51006, t51014, t51026, t51028, t51042, t51047) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3027(t14923, t14927, t10811, t14697, t40672, t828, t10905, t14825, t14829, t14819, t40517, t14910);
        let (t51049, t51055, t51058, t51060, t51070) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3028(t4423, t836, t14741, t2710, t2713, t10744, t14861, t808, t40791, t4442, t14468, t236, t807, t854);
    (t51000, t51006, t51014, t51026, t51028, t51042, t51047, t51049, t51055, t51058, t51060, t51070)
}
