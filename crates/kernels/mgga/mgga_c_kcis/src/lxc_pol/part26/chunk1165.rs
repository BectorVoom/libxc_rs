//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1165/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1165<F: Float>(t1464: F, t1489: F, t7392: F, t98310: F, t1497: F, t28503: F, t60756: F, t28720: F, t6140: F, t29365: F, t4142: F, t102275: F, t102280: F, t102286: F, t102292: F, t102295: F, t23102: F, t27567: F, t27583: F, t28714: F, t28816: F, t7971: F, t94966: F, t99046: F) -> (F, F, F, F) {
    let t102299 = t1464 * t98310 * t7392 * t1489;
    let t102303 = t1464 * t28503 * t60756 * t1497;
    let t102305 = t28720 * t6140;
    let t102308 = t4142 * t29365;
    let t102310 = -0.46336805555555555556e-3 * t27583 * t99046 * t23102 + 0.30891203703703703704e-3 * t27583 * t102275 - 0.61890573922526041666e-5 * t94966 * t102280 + 0.41224311342592592592e-4 * t27567 * t102275 - 0.23168402777777777778e-3 * t27583 * t102286 - 0.69505208333333333334e-3 * t28714 * t28816 + 0.46429444444444444444e-2 * t102292 - 0.30918233506944444445e-4 * t102295 - 0.10446625e-1 * t102299 + 0.23214722222222222221e-2 * t102303 - 0.24734586805555555555e-3 * t102305 * t7971 - 0.41270617283950617283e-2 * t102308;
    (t102299, t102303, t102308, t102310)
}
