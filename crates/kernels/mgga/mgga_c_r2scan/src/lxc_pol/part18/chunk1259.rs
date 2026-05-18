//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1259/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1259<F: Float>(t3579: F, t39274: F, t31498: F, t3263: F, t3275: F, t2867: F, t40324: F, t11622: F, t40713: F, t12396: F, t37282: F, t2847: F, t3582: F) -> (F, F, F, F, F, F) {
    let t43968 = t3579 * t39274 / F::new(2.0);
    let t43971 = t3275 * t3263 * t31498 / F::new(4.0);
    let t43974 = t3275 * t40324 * t2867 / F::new(2.0);
    let t43976 = F::new(45.0) / F::new(32.0) * t40713 * t11622;
    let t43978 = F::new(15.0) / F::new(8.0) * t37282 * t12396;
    let t43979 = t3582 * t2847;
    (t43968, t43971, t43974, t43976, t43978, t43979)
}
