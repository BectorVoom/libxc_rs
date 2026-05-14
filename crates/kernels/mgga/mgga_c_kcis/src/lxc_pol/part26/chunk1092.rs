//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1092/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1092<F: Float>(t26494: F, t7633: F, t26477: F, t26508: F, t26480: F, t92022: F, t92025: F, t92027: F, t92029: F, t92031: F, t92036: F, t92039: F, t92042: F, t92045: F, t92047: F, t92049: F, t92052: F, t92056: F) -> (F,) {
    let t92058 = t7633 * t26494;
    let t92060 = t26508 * t26477;
    let t92062 = t26480 * t26494;
    let t92064 = 0.49555782539766601562e-5 * t92022 + 0.2164276345486111111e-2 * t92025 + 0.19478487109375e-2 * t92027 + 0.111403033060546875e-3 * t92029 + 0.97307291666666666666e-2 * t92031 + 0.41703125000000000001e-2 * t92036 - 0.72223580246913580243e-1 * t92039 - 0.55652820312500000001e-3 * t92042 - 0.557015165302734375e-4 * t92045 + 0.55701516530273437501e-4 * t92047 + 0.55652820312500000001e-3 * t92049 - 0.41703125000000000001e-2 * t92052 + 0.92754700520833333333e-4 * t92056 + 0.208515625e-2 * t92058 - 0.83479230468750000001e-3 * t92060 - 0.64928290364583333333e-3 * t92062;
    (t92064,)
}
