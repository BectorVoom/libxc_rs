//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta476 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1460;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1461;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta476(t11262: f64, t3127: f64, t6262: f64, t3160: f64, t65338: f64, t1062: f64, t19463: f64, t15711: f64, t4834: f64, t1041: f64, t6301: f64, t3150: f64, t6307: f64, t3201: f64, t6318: f64, t1011: f64, t6292: f64, t697: f64, t19649: f64, t372: f64, t6284: f64, t6288: f64, t3091: f64, t43240: f64, t6267: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t65596, t65654, t65717, t65859, t66022, t66029) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1460(t11262, t3127, t6262, t3160, t65338, t1062, t19463, t15711, t4834, t1041, t6301, t3150, t6307);
        let (t66141, t66218, t66306, t66547, t66721, t66763) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1461(t3201, t6318, t1011, t6292, t697, t19649, t372, t6284, t6288, t3091, t43240, t6267);
    (t65596, t65654, t65717, t65859, t66022, t66029, t66141, t66218, t66306, t66547, t66721, t66763)
}
