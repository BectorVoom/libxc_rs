//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1249/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1249<F: Float>(t184: F, t563: F, t811: F, t2072: F, t1287: F, t813: F, t820: F, t1498: F, t2468: F, t1508: F, t2473: F, t1454: F, t6988: F, t1462: F, t1318: F, t1466: F, t6188: F) -> (F, F, F, F, F, F, F) {
    let t18555 = t811 * t563 * t184;
    let t18557 = 16.0 / 15.0 * t18555 * t2072;
    let t18561 = 8.0 / 15.0 * t1287 * t820 * t184 * t813;
    let t18563 = 4.0 / 15.0 * t1498 * t2468;
    let t18565 = 4.0 / 15.0 * t1508 * t2473;
    let t18567 = 8.0 / 45.0 * t6988 * t1454;
    let t18569 = 8.0 / 27.0 * t6988 * t1462;
    let t18573 = 4.0 / 15.0 * t1318 * t1466 * t6188 * t1287;
    (t18557, t18561, t18563, t18565, t18567, t18569, t18573)
}
