//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 928/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk928<F: Float>(t1429: F, t2365: F, t2366: F, t31747: F, t34777: F, t901: F, t35106: F, t41809: F, t475: F) -> (F, F, F, F) {
    let t41958 = t1429 * t2365 * t2366 * t31747;
    let t41960 = t34777 * t901;
    let t41962 = t35106 * t901;
    let t41965 = t41809 * t475;
    (t41958, t41960, t41962, t41965)
}
