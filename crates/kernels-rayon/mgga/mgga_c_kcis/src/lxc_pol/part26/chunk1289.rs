//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1289/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1289(t1610: f64, t27614: f64, t6176: f64, t7497: f64, t1394: f64, t5667: f64, t98409: f64, t102051: f64, t102054: f64, t102057: f64, t102061: f64, t102064: f64, t27595: f64, t29514: f64, t7978: f64, t94901: f64, t99152: f64, t99154: f64, t99157: f64, t99173: f64) -> (f64, f64, f64) {
    let t102068 = t6176 * t27614 * t7497 * t1610;
    let t102072 = t1394 * t98409 * t5667;
    let t102074 = -0.92835860883789062501e-5_f64 * t94901 * t29514 + 0.51485339506172839507e-4_f64 * t99152 + 0.20594135802469135803e-3_f64 * t99154 - 0.77382407407407407408e-2_f64 * t102051 - 0.41270617283950617283e-2_f64 * t102054 + 0.15476481481481481481e-2_f64 * t102057 - 0.185671721767578125e-4_f64 * t27595 * t102061 + 0.20594135802469135803e-3_f64 * t102064 - 0.34752604166666666667e-3_f64 * t7978 * t102068 + 0.61905925925925925924e-2_f64 * t102072 - t99157 + t99173;
    (t102068, t102072, t102074)
}
