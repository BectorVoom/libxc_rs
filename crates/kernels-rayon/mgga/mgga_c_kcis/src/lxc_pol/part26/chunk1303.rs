//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1303/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1303(t29365: f64, t4142: f64, t102275: f64, t102280: f64, t102286: f64, t102292: f64, t102295: f64, t102299: f64, t102303: f64, t102305: f64, t23102: f64, t27567: f64, t27583: f64, t28714: f64, t28816: f64, t7971: f64, t94966: f64, t99046: f64) -> (f64, f64) {
    let t102308 = t4142 * t29365;
    let t102310 = -0.46336805555555555556e-3_f64 * t27583 * t99046 * t23102 + 0.30891203703703703704e-3_f64 * t27583 * t102275 - 0.61890573922526041666e-5_f64 * t94966 * t102280 + 0.41224311342592592592e-4_f64 * t27567 * t102275 - 0.23168402777777777778e-3_f64 * t27583 * t102286 - 0.69505208333333333334e-3_f64 * t28714 * t28816 + 0.46429444444444444444e-2_f64 * t102292 - 0.30918233506944444445e-4_f64 * t102295 - 0.10446625e-1_f64 * t102299 + 0.23214722222222222221e-2_f64 * t102303 - 0.24734586805555555555e-3_f64 * t102305 * t7971 - 0.41270617283950617283e-2_f64 * t102308;
    (t102308, t102310)
}
