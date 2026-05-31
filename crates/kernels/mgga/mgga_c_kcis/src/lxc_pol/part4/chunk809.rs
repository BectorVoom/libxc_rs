//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 809/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk809<F: Float>(t3438: F, t4984: F, t3437: F, t284: F, t374: F, t3217: F, t41: F, t4813: F, t5044: F, t5049: F, t5051: F, t5054: F, t5056: F, t5058: F, t5060: F, t5063: F, t5065: F, t5069: F, t5071: F) -> (F, F, F, F, F, F, F) {
    let t5073 = t3438 * t4984;
    let t5074 = t3437 * t5073;
    let t5076 = t374 * t284;
    let t5077 = t41 * t3217;
    let t5078 = t5077 * t4813;
    let t5079 = t5076 * t5078;
    let t5081 = -t5044 / F::cast_from(16.0_f64) + t5049 / F::cast_from(8.0_f64) - t5051 / F::cast_from(192.0_f64) + t5054 / F::cast_from(6.0_f64) - t5056 / F::cast_from(6.0_f64) + t5058 / F::cast_from(24.0_f64) + t5060 / F::cast_from(24.0_f64) - t5063 / F::cast_from(24.0_f64) - t5065 / F::cast_from(192.0_f64) + t5069 / F::cast_from(256.0_f64) - t5071 / F::cast_from(16.0_f64) + t5074 / F::cast_from(192.0_f64) - t5079 / F::cast_from(72.0_f64);
    (t5073, t5074, t5076, t5077, t5078, t5079, t5081)
}
