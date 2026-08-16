//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1264/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1264(t4455: f64, t613: f64, t27567: f64, t99291: f64, t11425: f64, t1616: f64, t28788: f64, t7974: f64, t27651: f64, t8218: f64, t98597: f64, t98603: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t99429 = t613 * t4455;
    let t99437 = 0.10306077835648148148e-4_f64 * t27567 * t99291;
    let t99446 = t1616 * t11425;
    let t99452 = 0.23168402777777777778e-3_f64 * t28788 * t7974;
    let t99476 = t8218 * t27651;
    let t99478 = 0.23214722222222222222e-2_f64 * t98597;
    let t99480 = 0.23214722222222222222e-2_f64 * t98603;
    (t99429, t99437, t99446, t99452, t99476, t99478, t99480)
}
