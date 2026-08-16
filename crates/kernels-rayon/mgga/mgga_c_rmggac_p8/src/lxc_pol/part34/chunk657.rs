//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 657/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk657(t124: f64, t338: f64, t22: f64, t235: f64, t504: f64, t7191: f64, t14267: f64, t71: f64, t2227: f64, t4616: f64, t5542: f64, t8601: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t36632 = t124 * t338;
    let t36634 = t235 * t36632 * t22;
    let t36639 = t504 * t7191;
    let t36938 = t14267 * t71;
    let t37423 = t4616 * t2227;
    let t38350 = t8601 * t5542;
    (t36632, t36634, t36639, t36938, t37423, t38350)
}
