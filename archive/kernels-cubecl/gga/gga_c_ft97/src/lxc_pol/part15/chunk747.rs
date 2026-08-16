//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 747/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk747<F: Float>(t20972: F, t605: F, t144: F, t13201: F, t17432: F, t17434: F, t17436: F, t17438: F, t17440: F, t17443: F, t1901: F, t20927: F, t20931: F, t20935: F, t20939: F, t20942: F, t20945: F, t446: F, t9457: F) -> (F, F, F) {
    let t20973 = t605 * t20972;
    let t20974 = t144 * t20973;
    let t20977 = -F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t17432 + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t17434 + t17436 / F::cast_from(9.0_f64) - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t17438 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t17440 - t17443 / F::cast_from(3.0_f64) + t1901 * t20927 / F::cast_from(3.0_f64) - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1901 * t20931 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1901 * t20935 - t446 * t20939 - t446 * t20942 + F::cast_from(2.0_f64) * t446 * t20945 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t13201 - t446 * t20974 / F::cast_from(3.0_f64) - t9457;
    (t20973, t20974, t20977)
}
