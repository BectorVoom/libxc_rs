//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1169/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1169(t29540: f64, t29564: f64, t29604: f64, t29622: f64, t29434: f64, t29436: f64, t29438: f64, t29440: f64, t29442: f64, t29444: f64, t29446: f64, t29448: f64, t29450: f64, t29452: f64, t29454: f64, t29456: f64) -> (f64, f64) {
    let t29624 = t29540 + t29564 + t29604 + t29622;
    let t29638 = 0.1875e0_f64 * t29434 - 0.20234375e-1_f64 * t29436 - 0.21583333333333333334e0_f64 * t29438 + 0.53958333333333333334e-1_f64 * t29440 + 0.4046875e-1_f64 * t29442 + 0.21583333333333333334e0_f64 * t29444 - 0.53958333333333333334e-1_f64 * t29446 - 0.68347222222222222224e0_f64 * t29448 + 0.28777777777777777778e0_f64 * t29450 - 0.4046875e-1_f64 * t29452 + 0.5e0_f64 * t29454 - 0.125e0_f64 * t29456;
    (t29624, t29638)
}
