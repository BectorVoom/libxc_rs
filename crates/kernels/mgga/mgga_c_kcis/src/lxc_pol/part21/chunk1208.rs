//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1208/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1208<F: Float>(t28171: F, t7784: F, t26966: F, t28160: F, t27023: F, t28190: F, t27003: F, t330: F, t5314: F, t7788: F, t7789: F, t8095: F, t92613: F, t93158: F, t93161: F, t96273: F, t96298: F, t96311: F, t97039: F) -> (F,) {
    let t97377 = 0.61782407407407407408e-3 * t28171 * t7784;
    let t97385 = 0.61782407407407407408e-3 * t26966 * t28160;
    let t97387 = 0.23168402777777777778e-3 * t28190 * t27023;
    let t97393 = -0.15445601851851851852e-3 * t28190 * t27003 - 0.38691203703703703703e-3 * t96273 + t93158 + t97377 - 0.23168402777777777778e-3 * t7788 * t5314 * t7789 * t330 + 0.33980324074074074074e-2 * t92613 * t8095 - t97385 + t97387 + 0.11349419753086419753e-1 * t96298 - 0.30945286961263020833e-5 * t93161 - 0.13901041666666666667e-2 * t7788 * t97039 + 0.38691203703703703704e-2 * t96311;
    (t97393,)
}
