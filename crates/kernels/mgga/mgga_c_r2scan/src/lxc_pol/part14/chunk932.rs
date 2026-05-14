//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 932/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk932<F: Float>(t12365: F, t374: F, t1039: F, t3570: F, t1149: F, t2449: F, t2333: F, t3492: F, t3718: F, t2332: F, t1269: F, t1275: F, t6660: F, t815: F, t312: F, t320: F, t6659: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12366 = t12365 * t374;
    let t12367 = t1039 * t3570;
    let t12368 = t2449 * t1149;
    let t14402 = t2333 * t3492;
    let t15059 = t2333 * t3718;
    let t19025 = t2332 * t2332;
    let t19026 = 1.0 / t19025;
    let t19141 = t1269 * t1275;
    let t19146 = t815 * t6660;
    let t19155 = t312 / t6659 / t320;
    (t12366, t12367, t12368, t14402, t15059, t19026, t19141, t19146, t19155)
}
