//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1032/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1032(t12087: f64, t184: f64, t209: f64, t3563: f64, t813: f64, t1287: f64, t563: f64, t2072: f64, t5041: f64, t12063: f64, t12066: f64, t12070: f64, t12075: f64, t12078: f64, t12082: f64, t12084: f64, t12085: f64, t12086: f64) -> (f64, f64, f64, f64, f64) {
    let t12088 = 16.0_f64 / 15.0_f64 * t12087;
    let t12092 = 4.0_f64 / 15.0_f64 * t3563 * t209 * t184 * t813;
    let t12096 = 4.0_f64 / 5.0_f64 * t1287 * t563 * t184 * t813;
    let t12098 = 4.0_f64 / 5.0_f64 * t5041 * t2072;
    let t12099 = t12063 + t12066 + t12070 - t12075 - t12078 + t12082 + t12084 + t12085 - t12086 + t12088 + t12092 + t12096 - t12098;
    (t12088, t12092, t12096, t12098, t12099)
}
