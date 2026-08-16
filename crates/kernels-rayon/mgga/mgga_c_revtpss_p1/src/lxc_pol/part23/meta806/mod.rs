//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta806 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2638;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2639;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta806(t1558: f64, t2482: f64, t2801: f64, t4526: f64, t136: f64, t2457: f64, t39680: f64, t6022: f64, t10073: f64, t18746: f64, t18742: f64, t10069: f64, t231: f64, t2782: f64, t2783: f64, t62868: f64, t18729: f64, t2470: f64, t2798: f64, t2723: f64, t4503: f64, t62760: f64, t6016: f64, t879: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t62881, t62907, t62909, t62920, t62922) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2638(t1558, t2482, t2801, t4526, t136, t2457, t39680, t6022, t10073, t18746, t18742, t10069);
        let (t62938, t62952, t62961, t62967) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2639(t231, t2782, t2783, t62868, t18729, t2470, t2798, t2723, t4503, t62760, t2482, t6016, t879);
    (t62881, t62907, t62909, t62920, t62922, t62938, t62952, t62961, t62967)
}
