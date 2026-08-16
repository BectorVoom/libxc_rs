//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta501 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1787;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta501(t1450: f64, t6816: f64, t6836: f64, t196: f64, t197: f64, t6773: f64, t5920: f64, t94: f64, t21663: f64, t38: f64, t5868: f64, t76: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t29494, t29498, t29506, t29508, t29513, t29532) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1787(t1450, t6816, t6836, t196, t197, t6773, t5920, t94, t21663, t38, t5868, t76);
    (t29494, t29498, t29506, t29508, t29513, t29532)
}
