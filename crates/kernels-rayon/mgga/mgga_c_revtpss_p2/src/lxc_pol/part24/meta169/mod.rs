//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta169 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk836;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta169(t3699: f64, t5819: f64, t1012: f64, t1225: f64, t5825: f64, t3692: f64, t344: f64, t5843: f64, t3618: f64, t6421: f64, t247: f64, t1264: f64, t6429: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6652, t6653, t6658, t6659, t6662, t6663, t6667, t6673, t6678) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk836(t3699, t5819, t1012, t1225, t5825, t3692, t344, t5843, t3618, t6421, t247, t1264, t6429);
    (t6652, t6653, t6658, t6659, t6662, t6663, t6667, t6673, t6678)
}
