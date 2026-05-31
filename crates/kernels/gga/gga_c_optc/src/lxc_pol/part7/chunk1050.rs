//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1050/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1050<F: Float>(t141: F, t2087: F, t21868: F, t2080: F, t2089: F, t654: F, t6919: F, t137: F, t136: F, t22752: F, t6910: F, t6941: F) -> (F, F, F, F, F) {
    let t22827 = t2087 * t141 * t21868;
    let t22830 = t2080 * t2089;
    let t22832 = t654 * t6919;
    let t22834 = t137 * t137;
    let t22835 = F::cast_from(1.0_f64) / t22834;
    let t22836 = t136 * t22835;
    let t22838 = t22836 * t141 * t22752;
    let t22841 = t6941 * t6910;
    (t22827, t22830, t22832, t22838, t22841)
}
