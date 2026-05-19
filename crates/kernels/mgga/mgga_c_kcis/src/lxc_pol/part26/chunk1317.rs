//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1317/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1317<F: Float>(t102081: F, t102280: F, t102563: F, t102568: F, t102575: F, t102582: F, t102586: F, t20984: F, t27567: F, t27583: F, t28758: F, t28765: F, t28807: F, t4440: F, t6159: F, t8222: F, t99219: F, t99301: F, t99556: F) -> F {
    let t102594 = -F::cast_from(0.17411041666666666666e-2_f64) * t102563 + F::cast_from(0.61782407407407407408e-3_f64) * t99219 * t8222 - F::cast_from(0.30918233506944444444e-4_f64) * t27567 * t102568 - F::cast_from(0.92754700520833333333e-4_f64) * t27567 * t102280 - F::cast_from(0.61836467013888888888e-4_f64) * t27567 * t102081 + t99556 - F::cast_from(0.23168402777777777778e-3_f64) * t27583 * t4440 * t28758 * t102575 - F::cast_from(0.23168402777777777778e-3_f64) * t27583 * t102582 - F::cast_from(0.51588271604938271603e-3_f64) * t102586 + F::cast_from(0.23168402777777777778e-3_f64) * t99301 * t28807 - F::cast_from(0.69505208333333333334e-3_f64) * t27583 * t6159 * t28765 * t20984;
    t102594
}
