//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 953/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk953<F: Float>(t9280: F, t9024: F, t9026: F, t9028: F, t9031: F, t9034: F, t9036: F, t9038: F, t9040: F, t9043: F, t9048: F, t9050: F, t9054: F, t9056: F, t9058: F) -> (F, F) {
    let t9281 = F::cast_from(6.0_f64) * t9280;
    let t9296 = F::cast_from(9.0_f64) / F::cast_from(4.0_f64) * t9024 - F::cast_from(15.0_f64) / F::cast_from(16.0_f64) * t9026 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t9028 - F::cast_from(3.0_f64) / F::cast_from(16.0_f64) * t9031 + F::cast_from(15.0_f64) / F::cast_from(16.0_f64) * t9034 - F::cast_from(9.0_f64) / F::cast_from(4.0_f64) * t9036 - F::cast_from(3.0_f64) / F::cast_from(8.0_f64) * t9038 + F::cast_from(3.0_f64) / F::cast_from(16.0_f64) * t9040 + F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t9043 - F::cast_from(3.0_f64) / F::cast_from(32.0_f64) * t9048 - F::cast_from(3.0_f64) / F::cast_from(32.0_f64) * t9050 + F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t9054 - F::cast_from(3.0_f64) * t9056 + F::cast_from(3.0_f64) / F::cast_from(64.0_f64) * t9058;
    (t9281, t9296)
}
