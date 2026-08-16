//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1028/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1028(t2320: f64, t41490: f64, t701: f64, t2447: f64, t626: f64, t41451: f64, t41456: f64, t41461: f64, t41466: f64, t41471: f64, t41475: f64, t41480: f64, t41484: f64, t41488: f64) -> (f64, f64, f64) {
    let t41492 = t701 * t2320 * t41490;
    let t41495 = t701 * t626 * t2447;
    let t41497 = -0.23834947128395061728e0_f64 * t41451 + 0.11917473564197530864e0_f64 * t41456 + 0.30644932022222222222e0_f64 * t41461 - 0.30644932022222222222e0_f64 * t41466 + 0.25537443351851851852e-1_f64 * t41471 + 0.34049924469135802469e-1_f64 * t41475 - 0.15322466011111111111e0_f64 * t41480 + 0.22983699016666666666e0_f64 * t41484 - 0.38306165027777777778e-1_f64 * t41488 - 0.51074886703703703704e-1_f64 * t41492 + 0.17024962234567901234e-1_f64 * t41495;
    (t41492, t41495, t41497)
}
