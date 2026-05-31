//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 155/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk155<F: Float>(t373: F, t376: F, t379: F, t383: F) -> (F, F, F) {
    let t411 = F::cast_from(0.51785e1_f64) * t376 + F::cast_from(0.905775e0_f64) * t373 + F::cast_from(0.1100325e0_f64) * t379 + F::cast_from(0.1241775e0_f64) * t383;
    let t414 = F::cast_from(1.0_f64) + F::cast_from(0.29608574643216675549e2_f64) / t411;
    let t415 = F::ln(t414);
    (t411, t414, t415)
}
