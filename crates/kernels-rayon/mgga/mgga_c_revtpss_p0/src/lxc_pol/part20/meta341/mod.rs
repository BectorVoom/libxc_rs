//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta341 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1267;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta341(t1012: f64, t11821: f64, t3133: f64, t357: f64, t3059: f64, t3075: f64, t11670: f64, t4890: f64, t3317: f64, t3299: f64, t12047: f64, t15905: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16012, t16020, t16025, t16043, t16048, t16049, t16052, t16067) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1267(t1012, t11821, t3133, t357, t3059, t3075, t11670, t4890, t3317, t3299, t12047, t15905);
    (t16012, t16020, t16025, t16043, t16048, t16049, t16052, t16067)
}
