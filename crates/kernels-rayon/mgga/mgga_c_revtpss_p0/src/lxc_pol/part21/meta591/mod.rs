//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta591 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2308;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta591(t1260: f64, t17307: f64, t17183: f64, t5330: f64, t1774: f64, t3736: f64, t1811: f64, t3766: f64, t460: f64, t3781: f64, t3302: f64, t471: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t21275, t21306, t21389, t21451, t21452, t21455, t21456, t21471) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2308(t1260, t17307, t17183, t5330, t1774, t3736, t1811, t3766, t460, t3781, t3302, t471);
    (t21275, t21306, t21389, t21451, t21452, t21455, t21456, t21471)
}
