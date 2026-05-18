//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1001/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1001<F: Float>(t12024: F, t3275: F, t11465: F, t3579: F, t11555: F, t3472: F, t11336: F, t3270: F, t986: F, t3269: F, t11325: F, t3582: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t12025 = t3275 * t12024;
    let t12026 = F::new(45.0) / F::new(64.0) * t12025;
    let t12027 = t3579 * t11465;
    let t12028 = F::new(5.0) / F::new(16.0) * t12027;
    let t12029 = t3472 * t11555;
    let t12030 = t3275 * t12029;
    let t12031 = F::new(5.0) / F::new(16.0) * t12030;
    let t12033 = t3270 * t11336 * t986;
    let t12034 = t3269 * t12033;
    let t12035 = t12034 / F::new(4.0);
    let t12037 = t3275 * t11325 * t3582;
    (t12025, t12026, t12027, t12028, t12029, t12030, t12031, t12033, t12034, t12035, t12037)
}
