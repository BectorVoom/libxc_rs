//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1284/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1284<F: Float>(t100142: F, t100145: F, t100148: F, t100174: F, t101003: F, t101012: F, t101018: F, t101028: F, t18508: F, t26679: F, t26685: F, t26748: F, t27832: F, t27958: F, t28939: F, t4947: F, t7703: F) -> F {
    let t101031 = -F::cast_from(0.69505208333333333333e-3_f64) * t7703 * t101003 + F::cast_from(0.46336805555555555556e-3_f64) * t26748 * t28939 + F::cast_from(0.46336805555555555557e-3_f64) * t27832 * t27958 - F::cast_from(0.27636574074074074073e-2_f64) * t100142 + F::cast_from(0.61836467013888888889e-4_f64) * t26685 * t101012 + F::cast_from(0.18424382716049382715e-2_f64) * t100145 - F::cast_from(0.16581944444444444444e-1_f64) * t100148 + F::cast_from(0.12367293402777777778e-3_f64) * t26685 * t101018 + F::cast_from(0.46336805555555555556e-3_f64) * t7703 * t4947 * t26679 * t18508 - F::cast_from(0.33163888888888888888e-2_f64) * t100174 + F::cast_from(0.30918233506944444445e-4_f64) * t26685 * t101028;
    t101031
}
