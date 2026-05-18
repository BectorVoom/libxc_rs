//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 827/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk827<F: Float>(t1: F, t297: F, t7835: F, t313: F, t2586: F, t2590: F, t893: F, t2597: F, t6541: F, t897: F, t894: F, t224: F, t2269: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7837 = t7835 * t1 * t297;
    let t7838 = t313 * t7837;
    let t7845 = t2586 * t2590;
    let t7846 = t893 * t7845;
    let t7848 = t2586 * t2597;
    let t7849 = t893 * t7848;
    let t7851 = t897 * t6541;
    let t7852 = t894 * t7851;
    let t7856 = F::new(1.0) / t224 / t2269;
    (t7837, t7838, t7845, t7846, t7848, t7849, t7851, t7852, t7856)
}
