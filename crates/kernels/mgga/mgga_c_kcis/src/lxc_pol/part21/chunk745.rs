//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 745/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk745<F: Float>(t9235: F, t9265: F, t2531: F, t782: F, t781: F, t142: F, t2539: F, t826: F, t2626: F, t9024: F, t9026: F, t9028: F, t9031: F, t9034: F, t9036: F, t9038: F, t9040: F, t9043: F, t9048: F, t9050: F, t9054: F, t9056: F, t9058: F) -> (F, F, F, F, F, F, F, F) {
    let t9266 = t9235 + t9265;
    let t9268 = t2531 * t782;
    let t9273 = t781 * t781;
    let t9274 = 1.0 / t9273;
    let t9275 = t142 * t9274;
    let t9276 = t2539 * t826;
    let t9279 = t826 * t2626;
    let t9296 = 9.0 / 4.0 * t9024 - 15.0 / 16.0 * t9026 + 3.0 / 2.0 * t9028 - 3.0 / 16.0 * t9031 + 15.0 / 16.0 * t9034 - 9.0 / 4.0 * t9036 - 3.0 / 8.0 * t9038 + 3.0 / 16.0 * t9040 + 3.0 / 4.0 * t9043 - 3.0 / 32.0 * t9048 - 3.0 / 32.0 * t9050 + 3.0 / 4.0 * t9054 - 3.0 * t9056 + 3.0 / 64.0 * t9058;
    (t9266, t9268, t9273, t9274, t9275, t9276, t9279, t9296)
}
