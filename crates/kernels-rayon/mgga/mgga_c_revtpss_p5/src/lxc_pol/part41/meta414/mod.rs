//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta414 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1465;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1466;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta414(t1518: f64, t648: f64, t4292: f64, t94: f64, t1513: f64, t665: f64, t93: f64, t5920: f64, t1501: f64, t2175: f64, t2289: f64, t2339: f64, t625: f64, t8260: f64, t8264: f64, t655: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t27123, t27126, t28036, t28219, t29508, t30138, t30143, t31026, t31027) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1465(t1518, t648, t4292, t94, t1513, t665, t93, t5920, t1501, t2175, t2289, t2339, t625);
        let (t31028, t31030, t31032) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1466(t31027, t8260, t625, t8264, t655);
    (t27123, t27126, t28036, t28219, t29508, t30138, t30143, t31026, t31027, t31028, t31030, t31032)
}
