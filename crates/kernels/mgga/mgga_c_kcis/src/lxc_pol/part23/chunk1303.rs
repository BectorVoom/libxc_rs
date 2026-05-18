//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1303/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1303<F: Float>(t27567: F, t99291: F, t11425: F, t1616: F, t28788: F, t7974: F, t12617: F, t16685: F, t16694: F, t27583: F, t28701: F, t28758: F, t28805: F, t3797: F, t6151: F, t6159: F, t94931: F, t95088: F, t98543: F, t98553: F, t99224: F, t99419: F) -> F {
    let t99437 = F::new(0.10306077835648148148e-4) * t27567 * t99291;
    let t99446 = t1616 * t11425;
    let t99452 = F::new(0.23168402777777777778e-3) * t28788 * t7974;
    let t99461 = t99437 + F::new(0.11584201388888888889e-3) * t27583 * t99224 - F::new(0.46377350260416666666e-4) * t27567 * t99419 + F::new(0.23168402777777777778e-3) * t27583 * t6159 * t28758 * t16685 + F::new(0.92673611111111111112e-3) * t27583 * t6151 * t99446 * t16694 - t99452 + F::new(0.15445601851851851852e-3) * t27583 * t12617 * t28805 * t3797 + F::new(0.30918233506944444444e-4) * t94931 * t28701 - F::new(0.38691203703703703703e-3) * t98543 - F::new(0.11607361111111111111e-2) * t98553 + t95088;
    t99461
}
