//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1357/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1357(t82307: f64, t870: f64, t1914: f64, t40772: f64, t10140: f64, t25: f64, t193: f64, t9458: f64, t10121: f64, t22960: f64, t46240: f64, t1877: f64, t1915: f64, t1916: f64, t22959: f64, t23286: f64, t23290: f64, t23295: f64, t23296: f64, t23299: f64, t23302: f64, t4314: f64, t606: f64, t6670: f64, t6671: f64, t81521: f64, t81525: f64, t81529: f64, t81539: f64, t81543: f64, t81548: f64, t9257: f64) -> (f64, f64, f64, f64) {
    let t82308 = t82307 * t870;
    let t82312 = t1914 * t40772;
    let t82313 = t25 * t10140;
    let t82320 = t193 * t9458;
    let t82323 = t25 * t10121;
    let t82330 = t22960 * t46240;
    let t82333 = 3.0_f64 * t1877 * t23295 * t81521 - 3.0_f64 / 2.0_f64 * t1877 * t81525 * t6671 - 3.0_f64 / 2.0_f64 * t1877 * t6670 * t81529 - 3.0_f64 * t1877 * t23290 * t23299 + t1877 * t1915 * t9257 / 2.0_f64 + 3.0_f64 * t1877 * t81539 * t23296 + 9.0_f64 * t4314 * t1915 * t81543 - 9.0_f64 * t22959 * t81548 + t1877 * t82308 * t25 / 2.0_f64 - 3.0_f64 * t1877 * t82312 * t82313 + 3.0_f64 / 2.0_f64 * t1877 * t23286 * t606 + 3.0_f64 * t82320 * t1916 - t1877 * t6670 * t82323 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t1877 * t23290 * t23302 - 9.0_f64 / 2.0_f64 * t22959 * t82330;
    (t82308, t82312, t82320, t82333)
}
