//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta447 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1972;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta447(t13610: f64, t13638: f64, t13663: f64, t14308: f64, t1532: f64, t2609: f64, t10437: f64, t2398: f64, t4308: f64, t4305: f64, t262: f64, t4343: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t14310, t14312, t14313, t14315, t14317, t14318) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1972(t13610, t13638, t13663, t14308, t1532, t2609, t10437, t2398, t4308, t4305, t262, t4343);
    (t14310, t14312, t14313, t14315, t14317, t14318)
}
