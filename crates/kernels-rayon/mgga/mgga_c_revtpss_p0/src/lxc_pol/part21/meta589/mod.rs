//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta589 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2306;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta589(t19979: f64, t372: f64, t1651: f64, t2857: f64, t2852: f64, t1774: f64, t3362: f64, t1794: f64, t3617: f64, t17394: f64, t4890: f64, t3767: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t19980, t20094, t20099, t20921, t20945, t21013, t21014) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2306(t19979, t372, t1651, t2857, t2852, t1774, t3362, t1794, t3617, t17394, t4890, t3767);
    (t19980, t20094, t20099, t20921, t20945, t21013, t21014)
}
