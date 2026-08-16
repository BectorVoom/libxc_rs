//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta608 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1947;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta608(t11064: f64, t1468: f64, t27384: f64, t605: f64, t6079: f64, t5824: f64, t890: f64, t6075: f64, t27383: f64, t18392: f64, t30: f64, t1583: f64, t4343: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t106590, t106593, t106602, t106606, t106610, t106611, t106618, t106625) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1947(t11064, t1468, t27384, t605, t6079, t5824, t890, t6075, t27383, t18392, t30, t1583, t4343);
    (t106590, t106593, t106602, t106606, t106610, t106611, t106618, t106625)
}
