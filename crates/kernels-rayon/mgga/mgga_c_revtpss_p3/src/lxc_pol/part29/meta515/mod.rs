//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta515 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1837;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta515(t233: f64, t41077: f64, t1955: f64, t92888: f64, t7056: f64, t9646: f64, t1954: f64, t39643: f64, t2453: f64, t25309: f64, t25304: f64, t251: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t93118, t93126, t93134, t93139, t93140, t93157, t93160, t93169) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1837(t233, t41077, t1955, t92888, t7056, t9646, t1954, t39643, t2453, t25309, t25304, t251);
    (t93118, t93126, t93134, t93139, t93140, t93157, t93160, t93169)
}
