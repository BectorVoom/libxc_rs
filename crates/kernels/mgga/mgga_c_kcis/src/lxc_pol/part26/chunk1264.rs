//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1264/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1264<F: Float>(t4455: F, t613: F, t27567: F, t99291: F, t11425: F, t1616: F, t28788: F, t7974: F, t27651: F, t8218: F, t98597: F, t98603: F) -> (F, F, F, F, F, F, F) {
    let t99429 = t613 * t4455;
    let t99437 = F::new(0.10306077835648148148e-4) * t27567 * t99291;
    let t99446 = t1616 * t11425;
    let t99452 = F::new(0.23168402777777777778e-3) * t28788 * t7974;
    let t99476 = t8218 * t27651;
    let t99478 = F::new(0.23214722222222222222e-2) * t98597;
    let t99480 = F::new(0.23214722222222222222e-2) * t98603;
    (t99429, t99437, t99446, t99452, t99476, t99478, t99480)
}
