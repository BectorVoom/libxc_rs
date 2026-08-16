//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta633 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2329;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta633(t10578: f64, t9575: f64, t9572: f64, t2434: f64, t2496: f64, t2629: f64, t676: f64, t9419: f64, t9866: f64, t9863: f64, t762: f64, t9291: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t39423, t39425, t39427, t39429, t39430, t39432, t39433, t39438, t39440) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2329(t10578, t9575, t9572, t2434, t2496, t2629, t676, t9419, t9866, t9863, t762, t9291);
    (t39423, t39425, t39427, t39429, t39430, t39432, t39433, t39438, t39440)
}
