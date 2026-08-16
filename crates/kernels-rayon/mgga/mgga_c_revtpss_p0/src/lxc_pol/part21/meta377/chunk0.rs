//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1788/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1788(t1134: f64, t3390: f64, t3399: f64, t3407: f64, t12295: f64, t11335: f64, t281: f64, t414: f64, t1139: f64, t12322: f64, t12297: f64, t12299: f64, t12301: f64, t12303: f64, t12307: f64, t12310: f64, t12314: f64, t12317: f64, t12320: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12343 = t3390 * t1134;
    let t12344 = t12343 * t3399;
    let t12346 = t3407 * t1134;
    let t12347 = t12346 * t3399;
    let t12349 = 0.93011851851851851854e0_f64 * t12295;
    let t12351 = t281 * t11335 * t414;
    let t12352 = 0.36514074074074074075e0_f64 * t12351;
    let t12354 = t1139 * t12322;
    let t12356 = 0.19931111111111111111e0_f64 * t12299 + 0.33218518518518518518e0_f64 * t12307 + 0.39862222222222222223e0_f64 * t12297 - 0.59793333333333333333e0_f64 * t12301 - 0.29896666666666666667e0_f64 * t12303 - 0.11958666666666666667e1_f64 * t12310 + 0.17938e1_f64 * t12314 + 0.29896666666666666667e0_f64 * t12320 - 0.28483875e1_f64 * t12344 + 0.46074375e0_f64 * t12347 - t12349 - t12352 + 0.17938e1_f64 * t12317 + 0.3071625e0_f64 * t12354;
    (t12344, t12347, t12349, t12351, t12352, t12354, t12356)
}
