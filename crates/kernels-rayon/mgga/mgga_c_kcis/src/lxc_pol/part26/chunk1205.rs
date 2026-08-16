//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1205/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1205(t209: f64, t7637: f64, t8920: f64, t7636: f64, t26494: f64, t7633: f64, t26477: f64, t26508: f64, t26480: f64, t92022: f64, t92025: f64, t92027: f64, t92029: f64, t92031: f64, t92036: f64, t92039: f64, t92042: f64, t92045: f64, t92047: f64, t92049: f64, t92052: f64) -> (f64, f64) {
    let t92055 = t209 * t7637 * t8920;
    let t92056 = t7636 * t92055;
    let t92058 = t7633 * t26494;
    let t92060 = t26508 * t26477;
    let t92062 = t26480 * t26494;
    let t92064 = 0.49555782539766601562e-5_f64 * t92022 + 0.2164276345486111111e-2_f64 * t92025 + 0.19478487109375e-2_f64 * t92027 + 0.111403033060546875e-3_f64 * t92029 + 0.97307291666666666666e-2_f64 * t92031 + 0.41703125000000000001e-2_f64 * t92036 - 0.72223580246913580243e-1_f64 * t92039 - 0.55652820312500000001e-3_f64 * t92042 - 0.557015165302734375e-4_f64 * t92045 + 0.55701516530273437501e-4_f64 * t92047 + 0.55652820312500000001e-3_f64 * t92049 - 0.41703125000000000001e-2_f64 * t92052 + 0.92754700520833333333e-4_f64 * t92056 + 0.208515625e-2_f64 * t92058 - 0.83479230468750000001e-3_f64 * t92060 - 0.64928290364583333333e-3_f64 * t92062;
    (t92055, t92064)
}
