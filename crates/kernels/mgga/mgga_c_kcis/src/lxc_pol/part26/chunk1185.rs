//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1185/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1185<F: Float>(t1363: F, t21453: F, t1494: F, t6927: F, t4134: F, t7202: F, t3960: F, t7028: F, t1628: F, t23253: F, t286: F, t69: F) -> (F, F, F, F, F, F) {
    let t60299 = t21453 * t1363;
    let t60756 = t1494 * t6927;
    let t60761 = t4134 * t7202;
    let t60780 = t7028 * t3960;
    let t60988 = t23253 * t1628;
    let t61287 = t69 * t286;
    (t60299, t60756, t60761, t60780, t60988, t61287)
}
