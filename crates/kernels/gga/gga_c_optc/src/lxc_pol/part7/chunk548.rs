//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 548/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk548<F: Float>(t2618: F, t2683: F, t902: F, t907: F, t334: F, t906: F, t317: F, t956: F) -> (F, F, F, F, F) {
    let t2684 = t2618 + t2683;
    let t2688 = t902 * t907;
    let t2693 = F::cast_from(1.0_f64) / t906 / t334;
    let t2694 = t317 * t2693;
    let t2695 = t956 * t956;
    (t2684, t2688, t2693, t2694, t2695)
}
