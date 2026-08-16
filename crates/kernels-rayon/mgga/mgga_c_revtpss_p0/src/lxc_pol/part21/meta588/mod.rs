//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta588 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2305;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta588(t1043: f64, t3155: f64, t12131: f64, t357: f64, t1651: f64, t905: f64, t16509: f64, t4891: f64, t16584: f64, t1062: f64, t15670: f64, t1668: f64, t3181: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t19634, t19639, t19705, t19738, t19741, t19878, t19979) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2305(t1043, t3155, t12131, t357, t1651, t905, t16509, t4891, t16584, t1062, t15670, t1668, t3181);
    (t19634, t19639, t19705, t19738, t19741, t19878, t19979)
}
