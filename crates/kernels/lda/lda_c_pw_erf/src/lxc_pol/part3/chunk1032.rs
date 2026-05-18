//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1032/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1032<F: Float>(t12087: F, t184: F, t209: F, t3563: F, t813: F, t1287: F, t563: F, t2072: F, t5041: F, t12063: F, t12066: F, t12070: F, t12075: F, t12078: F, t12082: F, t12084: F, t12085: F, t12086: F) -> (F, F, F, F, F) {
    let t12088 = F::new(16.0) / F::new(15.0) * t12087;
    let t12092 = F::new(4.0) / F::new(15.0) * t3563 * t209 * t184 * t813;
    let t12096 = F::new(4.0) / F::new(5.0) * t1287 * t563 * t184 * t813;
    let t12098 = F::new(4.0) / F::new(5.0) * t5041 * t2072;
    let t12099 = t12063 + t12066 + t12070 - t12075 - t12078 + t12082 + t12084 + t12085 - t12086 + t12088 + t12092 + t12096 - t12098;
    (t12088, t12092, t12096, t12098, t12099)
}
