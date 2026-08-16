//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 647/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk647(t1757: f64, t5063: f64, t1899: f64, t5062: f64, t1869: f64, t4581: f64, t5048: f64, t1799: f64, t1894: f64, t3293: f64, t5185: f64, t5184: f64) -> (f64, f64, f64, f64) {
    let t10381 = t5063 * t1757;
    let t10382 = t1899 * t10381;
    let t10383 = t5062 * t10382;
    let t10384 = t1869 * t10383;
    let t10386 = t4581 * t5048;
    let t10387 = t1799 * t10386;
    let t10389 = t3293 * t1894;
    let t10390 = t5185 * t10389;
    let t10391 = t5184 * t10390;
    (t10381, t10384, t10387, t10391)
}
