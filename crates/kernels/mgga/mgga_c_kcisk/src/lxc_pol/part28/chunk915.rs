//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 915/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk915<F: Float>(t17182: F, t7263: F, t1773: F, t1769: F, t7203: F, t4999: F, t7208: F, t7213: F, t2456: F, t4995: F, t2449: F, t2454: F, t3934: F, t649: F, t164: F, t2465: F) -> (F, F, F, F, F, F, F, F) {
    let t17183 = t17182 * t7263;
    let t17184 = t1773 * t17183;
    let t17187 = 0.35981577432354634426e-1 * t7203 * t1769;
    let t17208 = 0.11993859144118211475e-1 * t7208 * t4999;
    let t17218 = t7213 * t1769;
    let t17220 = t2456 * t4995;
    let t17222 = t2449 * t4995;
    let t17248 = t649 * t2454 * t3934;
    let t17276 = t164 * t2465;
    (t17184, t17187, t17208, t17218, t17220, t17222, t17248, t17276)
}
