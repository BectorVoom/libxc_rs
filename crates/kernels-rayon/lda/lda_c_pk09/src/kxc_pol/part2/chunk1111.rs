//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 1111/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk1111(t10932: f64, t10941: f64, t1129: f64, t1134: f64, t12293: f64, t1657: f64, t1662: f64, t208: f64, t213: f64, t2131: f64, t2137: f64, t2439: f64, t2444: f64, t2695: f64, t2700: f64, t2951: f64, t413: f64, t417: f64, t555: f64, t563: f64, t7: f64, t7568: f64, t9559: f64, t9568: f64) -> f64 {
    let t12296 = t7568 * t208 / 4.0_f64 + t2137 * t1129 / 4.0_f64 + t563 * t2439 / 4.0_f64 + t7 * t9559 / 4.0_f64 + t9568 * t413 / 8.0_f64 + t2444 * t1657 / 8.0_f64 + t1134 * t2695 / 8.0_f64 + t213 * t10932 / 8.0_f64 + t10941 * t555 / 8.0_f64 + t2700 * t2131 / 8.0_f64 + t1662 * t2951 / 8.0_f64 + t417 * t12293 / 8.0_f64;
    t12296
}
