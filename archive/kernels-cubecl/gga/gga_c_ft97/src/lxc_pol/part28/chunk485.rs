//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 485/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk485<F: Float>(t7312: F, t7369: F, t7239: F, t7366: F, t2112: F, t1369: F, t28: F, t586: F, t7339: F, t1985: F, t27: F, t89: F) -> (F, F, F, F, F, F, F, F) {
    let t7370 = t7369 * t7312;
    let t7372 = t7366 * t7239 * t7370;
    let t7374 = t2112 * t7312;
    let t7376 = t1369 * t28 * t7374;
    let t7378 = t586 * t7339;
    let t7380 = t1369 * t28 * t7378;
    let t7382 = t1985 * t7312;
    let t7384 = t89 * t27 * t7382;
    (t7370, t7372, t7374, t7376, t7378, t7380, t7382, t7384)
}
