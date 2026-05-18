//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 524/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk524<F: Float>(t1587: F, t552: F, t321: F, t6557: F, t333: F, t128: F, t5840: F, t305: F, t1926: F, t6444: F, t6376: F, t326: F) -> (F, F, F, F, F, F) {
    let t6570 = t552 * t1587;
    let t6583 = t6557 * t321;
    let t6586 = t6557 * t333;
    let t6589 = t128 * t5840;
    let t6590 = t305 * t6589;
    let t6592 = t6444 * t1926;
    let t6598 = t128 * t6376;
    let t6599 = t326 * t6598;
    (t6570, t6583, t6586, t6590, t6592, t6599)
}
