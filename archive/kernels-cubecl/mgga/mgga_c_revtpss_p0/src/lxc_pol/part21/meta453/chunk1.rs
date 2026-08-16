//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1983/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1983<F: Float>(t57: F, t4186: F, t83: F, t13312: F, t1491: F, t2251: F, t2258: F, t4335: F, t606: F, t770: F, t14455: F, zeta_threshold: F) -> (F, F) {
    let t155 = t57 <= zeta_threshold;
    let t14458 = t83 * t4186;
    let t14466 = piecewise3::<F>(t155, F::cast_from(0.0_f64), -F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t1491 * t2251 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t14458 * t606 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t4335 * t2258 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t770 * t13312);
    let t14468 = t14455 / F::cast_from(2.0_f64) + t14466 / F::cast_from(2.0_f64);
    (t14458, t14468)
}
