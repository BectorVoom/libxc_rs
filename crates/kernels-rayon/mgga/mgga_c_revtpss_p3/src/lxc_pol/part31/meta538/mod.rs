//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta538 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1923;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta538(t2035: f64, t29506: f64, t5920: f64, t94: f64, t1937: f64, t7732: f64, t7735: f64, t21663: f64, t38: f64, t25132: f64, t25137: f64, t5819: f64, t5825: f64, t6968: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t29507, t29508, t29510, t29512, t29513, t29524) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1923(t2035, t29506, t5920, t94, t1937, t7732, t7735, t21663, t38, t25132, t25137, t5819, t5825, t6968);
    (t29507, t29508, t29510, t29512, t29513, t29524)
}
