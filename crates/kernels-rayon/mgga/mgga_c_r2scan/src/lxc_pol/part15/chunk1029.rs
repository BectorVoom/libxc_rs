//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1029/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1029(t19026: f64, t986: f64, t2850: f64, t6897: f64, t560: f64, t8001: f64, t481: f64, t2182: f64, t775: f64, t113: f64, t7202: f64, t253: f64, t5134: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t23791 = t986 * t19026;
    let t23987 = t2850 * t6897;
    let t24031 = t8001 * t560;
    let t24035 = t8001 * t481;
    let t24039 = t2182 * t775;
    let t24059 = t7202 * t113;
    let t24063 = t5134 * t253;
    (t23791, t23987, t24031, t24035, t24039, t24059, t24063)
}
