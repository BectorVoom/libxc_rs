//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta670 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2471;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta670(t12166: f64, t989: f64, t16409: f64, t994: f64, t3057: f64, t4980: f64, t11223: f64, t3286: f64, t11200: f64, t11213: f64, t3046: f64, t4995: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t43420, t43432, t43438, t43443, t43446, t43450, t43453) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2471(t12166, t989, t16409, t994, t3057, t4980, t11223, t3286, t11200, t11213, t3046, t4995);
    (t43420, t43432, t43438, t43443, t43446, t43450, t43453)
}
