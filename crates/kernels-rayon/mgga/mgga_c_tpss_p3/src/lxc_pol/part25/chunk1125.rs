//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1125/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1125(t4192: f64, t4202: f64, t1082: f64, t5198: f64, t1089: f64, t12210: f64, t4205: f64, t4238: f64, t4258: f64, t242: f64, t3060: f64, t5249: f64) -> (f64, f64, f64, f64, f64) {
    let t15478 = 0.11696447245269292414e1_f64 * t4192 * t4202;
    let t15479 = t5198 * t1082;
    let t15481 = 0.35089341735807877242e1_f64 * t1089 * t15479;
    let t15482 = t4205 * t12210;
    let t15484 = 0.34631718211362927518e2_f64 * t1089 * t15482;
    let t15485 = t4258 * t4238;
    let t15488 = t242 * t3060 * t5249;
    (t15478, t15481, t15484, t15485, t15488)
}
