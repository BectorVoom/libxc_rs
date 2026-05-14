//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 827/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk827<F: Float>(t115: F, t1194: F, t4182: F, t113: F, t27: F, t4238: F, t55: F, t1183: F, t97: F, t1195: F, t98: F, t4194: F, t4200: F, t2789: F, t29: F, t563: F) -> (F, F, F, F, F, F, F) {
    let t10500 = 0.1397792 * t1194 * t4182 * t115;
    let t10505 = 0.00011806781668990758 * t113 * t4238 * t27 * t55 * t115;
    let t10506 = t1183 * t97;
    let t10509 = 0.04717548 * t10506 * t98 * t1195;
    let t10511 = 0.12580128 * t4194 * t4200;
    let t10512 = t2789 * t27;
    let t10515 = 0.1254 * t10512 * t29 * t563;
    (t10500, t10505, t10506, t10509, t10511, t10512, t10515)
}
