//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 732/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk732<F: Float>(t2160: F, t7018: F, t155: F, t2078: F, t693: F, t697: F, t127: F, t7003: F, t675: F, t146: F, t2002: F, t671: F) -> (F, F, F, F, F) {
    let t7019 = t7018 * t2160;
    let t7022 = t155 * t693 * t2078;
    let t7023 = t7022 * t697;
    let t7025 = t7003 * t127;
    let t7026 = t675 * t7025;
    let t7030 = t146 * t671 * t2002;
    (t7019, t7022, t7023, t7026, t7030)
}
