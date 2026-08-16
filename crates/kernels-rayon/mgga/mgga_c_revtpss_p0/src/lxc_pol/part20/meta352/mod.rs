//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta352 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1282;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1283;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta352(t10578: f64, t9575: f64, t9572: f64, t2434: f64, t2496: f64, t2629: f64, t676: f64, t9419: f64, t9866: f64, t123: f64, t2390: f64, t2630: f64, t9863: f64, t762: f64, t9291: f64, t2251: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t39424, t39426, t39427, t39429, t39430, t39432, t39434, t39436) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1282(t10578, t9575, t9572, t2434, t2496, t2629, t676, t9419, t9866, t123, t2390, t2630);
        let (t39437, t39439, t39440, t39442, t39443) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1283(t39436, t10578, t9863, t762, t9291, t2629, t2251);
    (t39424, t39426, t39427, t39429, t39430, t39432, t39434, t39437, t39439, t39440, t39442, t39443)
}
