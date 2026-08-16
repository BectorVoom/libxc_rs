//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 958/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk958(t10082: f64, t10098: f64, t314: f64, t306: f64, t305: f64, t9739: f64, t304: f64, t1215: f64, t2567: f64, t334: f64, t9602: f64, t1336: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10099 = t10082 + t10098;
    let t10100 = t314 * t10099;
    let t10101 = t10100 * t306;
    let t10104 = t305 * t9739;
    let t10105 = t304 * t10104;
    let t10108 = t2567 * t1215;
    let t10116 = t9602 * t334;
    let t10119 = t2567 * t1336;
    (t10101, t10104, t10105, t10108, t10116, t10119)
}
