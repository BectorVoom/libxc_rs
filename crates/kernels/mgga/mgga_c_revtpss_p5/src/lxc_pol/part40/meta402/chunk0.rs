//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1478/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1478<F: Float>(t114: F, t31142: F, t8315: F, t2366: F, t8311: F, t104: F, t2357: F, t2358: F, t2362: F, t31035: F, t31134: F, t31135: F, t31137: F, t31139: F, t8258: F, t8267: F) -> (F, F, F, F, F, F) {
    let t115 = F::cast_from(1.0_f64) < t114;
    let t31143 = t8315 * t31142;
    let t31146 = t8311 * t2366;
    let t31149 = t104 * t2357;
    let t31150 = t31149 * t2358;
    let t31153 = t8315 * t2362;
    let t31157 = piecewise3::<F>(t115, F::cast_from(0.0_f64), -t31134 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t31135 + F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t31137 - F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t31035 * t31139 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t8258 * t31143 + t8258 * t31146 / F::cast_from(4.0_f64) - F::cast_from(5.0_f64) / F::cast_from(36.0_f64) * t8267 * t31150 - F::cast_from(5.0_f64) / F::cast_from(24.0_f64) * t8267 * t31153);
    (t31143, t31146, t31149, t31150, t31153, t31157)
}
