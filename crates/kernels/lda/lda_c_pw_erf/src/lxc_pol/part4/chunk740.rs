//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 740/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk740<F: Float>(t4995: F, t5028: F, t582: F, t186: F, t211: F, t2072: F, t2104: F, t1284: F, t1386: F, t2120: F, t1287: F, t209: F, t184: F, t813: F, t549: F, t563: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t5029 = t4995 + t5028;
    let t5030 = t582 * t5029;
    let t5031 = t186 * t5030;
    let t5033 = 2.0 / 15.0 * t211 * t5031;
    let t5035 = 8.0 / 15.0 * t2104 * t2072;
    let t5037 = 8.0 / 15.0 * t1284 * t2072;
    let t5039 = 16.0 / 45.0 * t2120 * t1386;
    let t5040 = t1287 * t209;
    let t5041 = t5040 * t184;
    let t5043 = 4.0 / 15.0 * t5041 * t813;
    let t5044 = t549 * t563;
    (t5029, t5030, t5031, t5033, t5035, t5037, t5039, t5040, t5041, t5043, t5044)
}
