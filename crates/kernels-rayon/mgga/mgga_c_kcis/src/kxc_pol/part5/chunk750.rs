//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 750/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk750(t1371: f64, t167: f64, t5713: f64, t1939: f64, t25: f64, t493: f64, t1938: f64, t531: f64, t833: f64, t3984: f64, t3999: f64, t1380: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5714 = t1371 * t167;
    let t5715 = t5713 * t5714;
    let t5718 = t25 * t1939;
    let t5719 = t493 * t5718;
    let t5721 = t1938 * t531;
    let t5722 = t5721 * t833;
    let t5723 = t3984 * t5722;
    let t5726 = t3999 * t1938;
    let t5727 = t5726 * t1380;
    (t5714, t5715, t5718, t5719, t5722, t5723, t5726, t5727)
}
