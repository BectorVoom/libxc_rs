//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1032/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1032<F: Float>(t12964: F, t354: F, t1146: F, t3250: F, t2333: F, t3492: F, t3718: F, t2332: F, t6660: F, t815: F, t312: F, t320: F, t6659: F) -> (F, F, F, F, F, F, F) {
    let t12965 = t354 * t12964;
    let t12966 = t1146 * t3250;
    let t14402 = t2333 * t3492;
    let t15059 = t2333 * t3718;
    let t19025 = t2332 * t2332;
    let t19026 = F::new(1.0) / t19025;
    let t19146 = t815 * t6660;
    let t19155 = t312 / t6659 / t320;
    (t12965, t12966, t14402, t15059, t19026, t19146, t19155)
}
