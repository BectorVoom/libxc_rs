//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 514/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk514<F: Float>(t2124: F, t2128: F, t2135: F, t2138: F, t2141: F, t2144: F, t1409: F, t1412: F, t1429: F, t1435: F, t1439: F, t1521: F, t1531: F) -> (F, F, F, F, F, F, F) {
    let t2573 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t2124;
    let t2574 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t2128;
    let t2575 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t2135;
    let t2576 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t2138;
    let t2577 = F::cast_from(16.0_f64) / F::cast_from(135.0_f64) * t2141;
    let t2578 = F::cast_from(16.0_f64) / F::cast_from(135.0_f64) * t2144;
    let t2579 = t1409 - t1412 + t1429 + t1435 + t1439 - t1521 - t1531 - t2573 + t2574 - t2575 + t2576 + t2577 + t2578;
    (t2573, t2574, t2575, t2576, t2577, t2578, t2579)
}
