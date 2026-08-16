//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1349/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1349(t103045: f64, t7949: f64, t1532: f64, t7052: f64, t7299: f64, t94748: f64, t28594: f64, t5919: f64, t1928: f64, t2034: f64, t7953: f64, t103031: f64, t103033: f64, t103035: f64, t103038: f64, t103040: f64, t103043: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t103046 = t103045 * t7949;
    let t103048 = t1532 * t7052;
    let t103049 = t103048 * t7949;
    let t103051 = t94748 * t7299;
    let t103053 = t28594 * t5919;
    let t103055 = t2034 * t1928;
    let t103056 = t103055 * t7953;
    let t103058 = 0.9375e-1_f64 * t103031 - 0.1875e0_f64 * t103033 - 0.26979166666666666667e-1_f64 * t103035 - 0.9375e-1_f64 * t103038 + 0.5e0_f64 * t103040 - 0.9375e-1_f64 * t103043 + 0.5e0_f64 * t103046 - 0.91666666666666666667e0_f64 * t103049 + 0.53958333333333333334e-1_f64 * t103051 + 0.33333333333333333334e0_f64 * t103053 - 0.33333333333333333333e0_f64 * t103056;
    (t103046, t103049, t103051, t103053, t103056, t103058)
}
