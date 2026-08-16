//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta528 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1833;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta528(t1954: f64, t39643: f64, t7056: f64, t2453: f64, t25309: f64, t25304: f64, t251: f64, t25410: f64, t2438: f64, t837: f64, t2434: f64, t25374: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t93139, t93140, t93157, t93160, t93169, t93170, t93173, t93182, t93189, t93190) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1833(t1954, t39643, t7056, t2453, t25309, t25304, t251, t25410, t2438, t837, t2434, t25374);
    (t93139, t93140, t93157, t93160, t93169, t93170, t93173, t93182, t93189, t93190)
}
