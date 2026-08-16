//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1104/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1104(t10845: f64, t10864: f64, t2265: f64, t2405: f64, t2409: f64, t2413: f64, t2923: f64, t2939: f64, t2951: f64, t43146: f64, t43148: f64, t43150: f64, t43152: f64, t43158: f64, t43160: f64, t43162: f64, t43164: f64, t43177: f64, t904: f64, t9578: f64) -> f64 {
    let t43183 = -2.0_f64 * t2265 * t2923 * t2413 * t2951 - 8.0_f64 * t43146 - 16.0_f64 / 3.0_f64 * t43148 + 8.0_f64 / 9.0_f64 * t43150 + 8.0_f64 / 3.0_f64 * t43152 - 2.0_f64 / 3.0_f64 * t2265 * t10845 * t2405 * t2951 + 8.0_f64 / 3.0_f64 * t43158 - 40.0_f64 / 9.0_f64 * t43160 + 8.0_f64 / 3.0_f64 * t43162 + 2.0_f64 * t2265 * t43164 * t2405 * t2939 + 8.0_f64 / 3.0_f64 * t2265 * t10845 * t9578 * t904 + 4.0_f64 * t2265 * t2923 * t2409 * t2951 - 4.0_f64 / 9.0_f64 * t43177 + 6.0_f64 * t2265 * t10864 * t2413 * t2939;
    t43183
}
