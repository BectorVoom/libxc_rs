//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 793/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk793<F: Float>(t146: F, t147: F, t9083: F, t279: F, t5123: F, t5150: F, t5179: F, t5183: F, t6062: F, t7419: F, t7459: F, t7468: F, t7472: F, t7475: F, t7479: F, t7482: F, t8861: F, t8863: F, t8867: F, t8874: F) -> (F, F) {
    let t9085 = t146 * t147 * t9083;
    let t9088 = 0.58544643236296698112e-1 * t5123 + 0.81312004494856525156e-4 * t5150 + 0.23115257973478049502e0 * t8861 + 0.12805040077930161442e0 * t8863 - 0.23115257973478049502e0 * t8867 - 0.57829097596741960691e-3 * t7419 + 0.679213007128961539e-1 * t5179 + 0.2037639021386884617e0 * t5183 - t7459 - t7468 + t7472 - 0.13869154784086829701e1 * t8874 - t7475 - t7479 - t7482 + 0.43341108700271342816e-1 * t9085 * t279 - t6062;
    (t9085, t9088)
}
