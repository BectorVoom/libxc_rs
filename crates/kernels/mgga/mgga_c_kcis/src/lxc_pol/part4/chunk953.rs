//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 953/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk953<F: Float>(t9280: F, t9024: F, t9026: F, t9028: F, t9031: F, t9034: F, t9036: F, t9038: F, t9040: F, t9043: F, t9048: F, t9050: F, t9054: F, t9056: F, t9058: F) -> (F, F) {
    let t9281 = F::new(6.0) * t9280;
    let t9296 = F::new(9.0) / F::new(4.0) * t9024 - F::new(15.0) / F::new(16.0) * t9026 + F::new(3.0) / F::new(2.0) * t9028 - F::new(3.0) / F::new(16.0) * t9031 + F::new(15.0) / F::new(16.0) * t9034 - F::new(9.0) / F::new(4.0) * t9036 - F::new(3.0) / F::new(8.0) * t9038 + F::new(3.0) / F::new(16.0) * t9040 + F::new(3.0) / F::new(4.0) * t9043 - F::new(3.0) / F::new(32.0) * t9048 - F::new(3.0) / F::new(32.0) * t9050 + F::new(3.0) / F::new(4.0) * t9054 - F::new(3.0) * t9056 + F::new(3.0) / F::new(64.0) * t9058;
    (t9281, t9296)
}
