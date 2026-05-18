//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1020/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1020<F: Float>(t12019: F, t374: F, t2333: F, t3347: F, t1064: F, t6897: F, t3617: F, t2332: F, t1269: F, t1275: F, t6660: F, t815: F) -> (F, F, F, F, F, F, F) {
    let t12020 = t12019 * t374;
    let t13908 = t2333 * t3347;
    let t14160 = t6897 * t1064;
    let t14656 = t2333 * t3617;
    let t19025 = t2332 * t2332;
    let t19026 = F::new(1.0) / t19025;
    let t19141 = t1269 * t1275;
    let t19146 = t815 * t6660;
    (t12020, t13908, t14160, t14656, t19026, t19141, t19146)
}
