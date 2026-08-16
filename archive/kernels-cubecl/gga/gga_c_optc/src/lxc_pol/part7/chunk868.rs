//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 868/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk868<F: Float>(t2347: F, t984: F, t2351: F, t988: F, t355: F, t7592: F, t7529: F, t7538: F, t7541: F, t7544: F, t7547: F, t7560: F, t7563: F, t7566: F, t7596: F, t7599: F) -> (F, F, F, F) {
    let t8304 = t984 * t2347;
    let t8306 = t988 * t2351;
    let t8307 = t355 * t8306;
    let t8319 = F::cast_from(0.54733333333333333333e-2_f64) * t7592;
    let t8320 = -F::cast_from(0.4926e-2_f64) * t7560 + F::cast_from(0.2463e-2_f64) * t7563 - F::cast_from(0.12315e-2_f64) * t7596 - F::cast_from(0.7389e-2_f64) * t7566 + F::cast_from(0.7389e-2_f64) * t7599 - F::cast_from(0.38776666666666666665e1_f64) * t7529 + F::cast_from(0.77553333333333333331e1_f64) * t7538 - F::cast_from(0.38776666666666666665e1_f64) * t7541 - F::cast_from(0.11633e2_f64) * t7544 + F::cast_from(0.11633e2_f64) * t7547 - t8319;
    (t8304, t8306, t8307, t8320)
}
