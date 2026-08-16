//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta539 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2348;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2349;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta539(t17528: f64, t3594: f64, t1214: f64, t4186: f64, t5296: f64, t1042: f64, t1469: f64, t3584: f64, t3172: f64, t5286: f64, t1247: f64, t3707: f64, t5292: f64, t12268: f64, t3617: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17529, t17534, t17535, t17536, t17539, t17540, t17541, t17544, t17546, t17547) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2348(t17528, t3594, t1214, t4186, t5296, t1042, t1469, t3584, t3172, t5286, t1247, t3707, t5292);
        let t17550 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2349(t12268, t3617);
    (t17529, t17534, t17535, t17536, t17539, t17540, t17541, t17544, t17546, t17547, t17550)
}
