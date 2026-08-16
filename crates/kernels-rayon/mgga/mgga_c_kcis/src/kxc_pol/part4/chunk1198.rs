//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1198/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1198(t3577: f64, t5233: f64, t1219: f64, t3569: f64, t5237: f64, t10865: f64, t1830: f64, t3551: f64, t5250: f64, t969: f64, t1835: f64, t3025: f64) -> (f64, f64, f64, f64, f64) {
    let t15327 = t5233 * t3577;
    let t15328 = t15327 * t1219;
    let t15331 = t5237 * t3569;
    let t15334 = t1830 * t10865;
    let t15335 = t15334 * t3551;
    let t15342 = t5250 * t969;
    let t15345 = t1835 * t3025;
    (t15328, t15331, t15335, t15342, t15345)
}
