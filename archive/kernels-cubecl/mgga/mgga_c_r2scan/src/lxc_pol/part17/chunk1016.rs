//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1016/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1016<F: Float>(t12725: F, t12567: F, t3469: F, t12422: F, t11189: F, t12574: F, t3275: F, t1039: F, t3787: F, t12098: F, t3582: F, t12414: F, t3465: F) -> (F, F, F, F, F, F, F) {
    let t12726 = F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t12725;
    let t12727 = t12567 * t3469;
    let t12728 = t12727 / F::cast_from(4.0_f64);
    let t12729 = t12422 * t3469;
    let t12730 = t12729 / F::cast_from(4.0_f64);
    let t12732 = t3275 * t11189 * t12574;
    let t12733 = F::cast_from(45.0_f64) / F::cast_from(64.0_f64) * t12732;
    let t12734 = t1039 * t3787;
    let t12735 = F::cast_from(2.0_f64) * t12734;
    let t12737 = t3275 * t12098 * t3582;
    let t12738 = F::cast_from(5.0_f64) / F::cast_from(8.0_f64) * t12737;
    let t12739 = t3465 * t12414;
    (t12726, t12728, t12730, t12733, t12735, t12738, t12739)
}
