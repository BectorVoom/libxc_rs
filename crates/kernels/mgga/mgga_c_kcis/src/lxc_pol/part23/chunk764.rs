//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 764/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk764<F: Float>(t9235: F, t9265: F, t2531: F, t782: F, t781: F, t142: F, t2539: F, t826: F, t2626: F, t9024: F, t9026: F, t9028: F, t9031: F, t9034: F, t9036: F, t9038: F, t9040: F, t9043: F, t9048: F, t9050: F, t9054: F, t9056: F, t9058: F) -> (F, F, F, F, F, F, F, F) {
    let t9266 = t9235 + t9265;
    let t9268 = t2531 * t782;
    let t9273 = t781 * t781;
    let t9274 = F::cast_from(1.0_f64) / t9273;
    let t9275 = t142 * t9274;
    let t9276 = t2539 * t826;
    let t9279 = t826 * t2626;
    let t9296 = F::cast_from(9.0_f64) / F::cast_from(4.0_f64) * t9024 - F::cast_from(15.0_f64) / F::cast_from(16.0_f64) * t9026 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t9028 - F::cast_from(3.0_f64) / F::cast_from(16.0_f64) * t9031 + F::cast_from(15.0_f64) / F::cast_from(16.0_f64) * t9034 - F::cast_from(9.0_f64) / F::cast_from(4.0_f64) * t9036 - F::cast_from(3.0_f64) / F::cast_from(8.0_f64) * t9038 + F::cast_from(3.0_f64) / F::cast_from(16.0_f64) * t9040 + F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t9043 - F::cast_from(3.0_f64) / F::cast_from(32.0_f64) * t9048 - F::cast_from(3.0_f64) / F::cast_from(32.0_f64) * t9050 + F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t9054 - F::cast_from(3.0_f64) * t9056 + F::cast_from(3.0_f64) / F::cast_from(64.0_f64) * t9058;
    (t9266, t9268, t9273, t9274, t9275, t9276, t9279, t9296)
}
