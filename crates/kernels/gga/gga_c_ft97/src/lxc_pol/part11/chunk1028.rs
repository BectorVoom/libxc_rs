//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1028/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1028<F: Float>(t2320: F, t41490: F, t701: F, t2447: F, t626: F, t41451: F, t41456: F, t41461: F, t41466: F, t41471: F, t41475: F, t41480: F, t41484: F, t41488: F) -> (F, F, F) {
    let t41492 = t701 * t2320 * t41490;
    let t41495 = t701 * t626 * t2447;
    let t41497 = -F::cast_from(0.23834947128395061728e0_f64) * t41451 + F::cast_from(0.11917473564197530864e0_f64) * t41456 + F::cast_from(0.30644932022222222222e0_f64) * t41461 - F::cast_from(0.30644932022222222222e0_f64) * t41466 + F::cast_from(0.25537443351851851852e-1_f64) * t41471 + F::cast_from(0.34049924469135802469e-1_f64) * t41475 - F::cast_from(0.15322466011111111111e0_f64) * t41480 + F::cast_from(0.22983699016666666666e0_f64) * t41484 - F::cast_from(0.38306165027777777778e-1_f64) * t41488 - F::cast_from(0.51074886703703703704e-1_f64) * t41492 + F::cast_from(0.17024962234567901234e-1_f64) * t41495;
    (t41492, t41495, t41497)
}
