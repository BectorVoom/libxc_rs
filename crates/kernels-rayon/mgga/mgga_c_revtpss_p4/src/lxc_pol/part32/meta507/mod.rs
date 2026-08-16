//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta507 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1796;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1797;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta507(t5920: f64, t93: f64, t1843: f64, t7983: f64, t5542: f64, t8108: f64, t2097: f64, t6861: f64, t4003: f64, t26079: f64, t26321: f64, t26324: f64, t26325: f64, t26328: f64, t27921: f64, t27926: f64, t27929: f64, t27953: f64, t27955: f64, t30048: f64, t30050: f64, t26310: f64, t26312: f64, t27924: f64, t27937: f64, t30035: f64, t30037: f64, t30039: f64, t30041: f64, t30043: f64, t30045: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t30143, t30209, t30218, t30225, t30226, t30227, t30246) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1796(t5920, t93, t1843, t7983, t5542, t8108, t2097, t6861, t4003, t26079, t26321, t26324, t26325, t26328, t27921, t27926, t27929, t27953, t27955, t30048, t30050);
        let t30247 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1797(t26310, t26312, t27924, t27937, t30035, t30037, t30039, t30041, t30043, t30045, t30246);
    (t30143, t30209, t30218, t30225, t30226, t30227, t30247)
}
