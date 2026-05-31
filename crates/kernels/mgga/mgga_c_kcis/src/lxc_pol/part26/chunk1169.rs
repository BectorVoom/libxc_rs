//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1169/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1169<F: Float>(t29540: F, t29564: F, t29604: F, t29622: F, t29434: F, t29436: F, t29438: F, t29440: F, t29442: F, t29444: F, t29446: F, t29448: F, t29450: F, t29452: F, t29454: F, t29456: F) -> (F, F) {
    let t29624 = t29540 + t29564 + t29604 + t29622;
    let t29638 = F::cast_from(0.1875e0_f64) * t29434 - F::cast_from(0.20234375e-1_f64) * t29436 - F::cast_from(0.21583333333333333334e0_f64) * t29438 + F::cast_from(0.53958333333333333334e-1_f64) * t29440 + F::cast_from(0.4046875e-1_f64) * t29442 + F::cast_from(0.21583333333333333334e0_f64) * t29444 - F::cast_from(0.53958333333333333334e-1_f64) * t29446 - F::cast_from(0.68347222222222222224e0_f64) * t29448 + F::cast_from(0.28777777777777777778e0_f64) * t29450 - F::cast_from(0.4046875e-1_f64) * t29452 + F::cast_from(0.5e0_f64) * t29454 - F::cast_from(0.125e0_f64) * t29456;
    (t29624, t29638)
}
