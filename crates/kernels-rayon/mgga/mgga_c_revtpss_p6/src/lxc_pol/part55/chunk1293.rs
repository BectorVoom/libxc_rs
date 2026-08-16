//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1293/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1293(t128920: f64, t128930: f64, t128932: f64, t128933: f64, t2055: f64, t2089: f64, t2322: f64, t28707: f64, t28734: f64, t28737: f64, t29337: f64, t29422: f64, t33287: f64, t33311: f64, t34821: f64, t4254: f64, t4297: f64, t651: f64, t7474: f64, t7586: f64, t7732: f64, t8152: f64, t8764: f64) -> f64 {
    let t131064 = -2.0_f64 * t2055 * t29337 * t651 - t2089 * t29422 - 2.0_f64 * t2322 * t34821 - t28707 * t8764 - 2.0_f64 * t28734 * t7586 - 2.0_f64 * t28737 * t7586 - 2.0_f64 * t33287 * t4297 - 2.0_f64 * t33311 * t7732 - 2.0_f64 * t34821 * t4254 - t7474 * t8152 - t128920 - t128930 - t128932 - t128933;
    t131064
}
