//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1084/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1084<F: Float>(t10417: F, t39535: F, t39555: F, t4041: F, t43001: F, t43008: F, t45601: F, t45603: F, t45608: F, t45610: F, t45614: F, t45617: F, t45630: F, t45633: F, t45636: F, t45641: F, t45646: F, t5928: F, t9427: F) -> F {
    let t48564 = -t43001 + F::cast_from(0.47896966807455234255e0_f64) * t39535 + t43008 - F::cast_from(0.212822999466489197e-4_f64) * t45601 - F::cast_from(0.1064114997332445985e-4_f64) * t45603 - F::cast_from(0.20496175532535769483e-3_f64) * t39555 + F::cast_from(0.85129199786595678799e-5_f64) * t45608 + F::cast_from(0.1702583995731913576e-4_f64) * t45610 + F::cast_from(0.5107751987195740728e-4_f64) * t45614 + F::cast_from(0.5107751987195740728e-4_f64) * t45617 + F::cast_from(0.11974241701863808564e0_f64) * t4041 * t10417 + F::cast_from(0.79828278012425390428e-1_f64) * t5928 * t9427 - F::cast_from(0.81300399444200075499e-3_f64) * t45630 - F::cast_from(0.162600798888400151e-2_f64) * t45633 - F::cast_from(0.81300399444200075499e-3_f64) * t45636 + F::cast_from(0.212822999466489197e-4_f64) * t45641 - F::cast_from(0.212822999466489197e-4_f64) * t45646;
    t48564
}
