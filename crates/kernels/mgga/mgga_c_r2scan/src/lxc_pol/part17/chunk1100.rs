//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1100/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1100<F: Float>(t12742: F, t37271: F, t11559: F, t12098: F, t3275: F, t12052: F, t12567: F, t3262: F, t3465: F, t43757: F, t11189: F, t43721: F, t3472: F, t43729: F, t11336: F, t37327: F, t42868: F) -> (F, F, F, F, F, F, F) {
    let t44904 = 5.0 / 8.0 * t37271 * t12742;
    let t44907 = 5.0 / 8.0 * t3275 * t12098 * t11559;
    let t44909 = t12567 * t12052 / 4.0;
    let t44912 = 3.0 / 2.0 * t3262 * t3465 * t43757;
    let t44915 = 45.0 / 64.0 * t3275 * t11189 * t43721;
    let t44918 = 15.0 / 16.0 * t3262 * t3472 * t43729;
    let t44921 = 15.0 / 8.0 * t37327 * t11336 * t42868;
    (t44904, t44907, t44909, t44912, t44915, t44918, t44921)
}
