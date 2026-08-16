//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta785 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2595;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2596;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta785(t18550: f64, t72: f64, t757: f64, t18299: f64, t750: f64, t18298: f64, t705: f64, t18281: f64, t706: f64, t18838: f64, t892: f64, t2609: f64, t2611: f64, t5819: f64, t18544: f64, t2398: f64, t14440: f64, t4311: f64, t14386: f64, t4305: f64, t177: f64, t762: f64, t123: f64, t2630: f64, t5941: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t61093, t61114, t61122, t61130, t61139, t61165) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2595(t18550, t72, t757, t18299, t750, t18298, t705, t18281, t706, t18838, t892, t2609, t2611, t5819);
        let (t61178, t61180, t61201, t61239, t61247) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2596(t18544, t2398, t14440, t4311, t14386, t4305, t177, t18550, t762, t123, t2630, t5941);
    (t61093, t61114, t61122, t61130, t61139, t61165, t61178, t61180, t61201, t61239, t61247)
}
