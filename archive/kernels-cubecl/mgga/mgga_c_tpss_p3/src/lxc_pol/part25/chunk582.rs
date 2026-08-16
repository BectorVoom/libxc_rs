//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 582/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk582<F: Float>(t1173: F, t1186: F, t14: F, t22: F, t498: F, t558: F, t563: F, t491: F, t494: F, t2140: F, t512: F, t1212: F, t756: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3209 = F::cast_from(8.0_f64) * t1173 * t1186;
    let t3211 = t14 * t22;
    let t3213 = F::cast_from(12.0_f64) * t3211 * t498;
    let t3214 = t558 * t563;
    let t3216 = F::cast_from(32.0_f64) * t3214 * t498;
    let t3217 = F::cast_from(1.0_f64) / t491;
    let t3225 = F::cast_from(1.0_f64) / t494;
    let t3239 = F::cast_from(35.0_f64) / F::cast_from(432.0_f64) * t2140 * t512;
    let t3240 = t756 * t1212;
    (t3209, t3211, t3213, t3214, t3216, t3217, t3225, t3239, t3240)
}
