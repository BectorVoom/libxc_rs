//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta976 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3284;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3285;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta976(t50873: f64, t40172: f64, t14330: f64, t18575: f64, t2258: f64, t14370: f64, t18259: f64, t18562: f64, t2626: f64, t18576: f64, t50895: f64, t5819: f64, t606: f64, t749: f64, t1522: f64, t49880: f64, t50878: f64, t40067: f64, t40072: f64, t40167: f64, t40171: f64, t40184: f64, t61310: f64, t61311: f64, t61313: f64, t61316: f64, t61317: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t62269, t62270, t62273, t62275, t62277, t62279, t62282) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3284(t50873, t40172, t14330, t18575, t2258, t14370, t18259, t18562, t2626, t18576, t50895, t5819, t606, t749);
        let (t62283, t62285, t62286, t62287) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3285(t62282, t1522, t49880, t50878, t40067, t40072, t40167, t40171, t40184, t61310, t61311, t61313, t61316, t61317, t62269, t62270, t62273, t62275, t62277, t62279);
    (t62269, t62270, t62273, t62275, t62277, t62279, t62283, t62285, t62286, t62287)
}
