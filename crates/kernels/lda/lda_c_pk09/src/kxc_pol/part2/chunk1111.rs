//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 1111/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk1111<F: Float>(t10932: F, t10941: F, t1129: F, t1134: F, t12293: F, t1657: F, t1662: F, t208: F, t213: F, t2131: F, t2137: F, t2439: F, t2444: F, t2695: F, t2700: F, t2951: F, t413: F, t417: F, t555: F, t563: F, t7: F, t7568: F, t9559: F, t9568: F) -> F {
    let t12296 = t7568 * t208 / F::cast_from(4.0_f64) + t2137 * t1129 / F::cast_from(4.0_f64) + t563 * t2439 / F::cast_from(4.0_f64) + t7 * t9559 / F::cast_from(4.0_f64) + t9568 * t413 / F::cast_from(8.0_f64) + t2444 * t1657 / F::cast_from(8.0_f64) + t1134 * t2695 / F::cast_from(8.0_f64) + t213 * t10932 / F::cast_from(8.0_f64) + t10941 * t555 / F::cast_from(8.0_f64) + t2700 * t2131 / F::cast_from(8.0_f64) + t1662 * t2951 / F::cast_from(8.0_f64) + t417 * t12293 / F::cast_from(8.0_f64);
    t12296
}
