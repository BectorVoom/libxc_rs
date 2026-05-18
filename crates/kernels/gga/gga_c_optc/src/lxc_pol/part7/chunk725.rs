//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 725/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk725<F: Float>(t6915: F, t136: F, t141: F, t6856: F, t131: F, t6165: F, t130: F, t142: F, t127: F, t2067: F, t616: F, t2034: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6916 = F::new(1.0) / t6915;
    let t6917 = t136 * t6916;
    let t6919 = t6917 * t141 * t6856;
    let t6922 = t131 * t6165;
    let t6923 = t130 * t6922;
    let t6925 = F::new(0.47892880429854730775e0) * t6923 * t142;
    let t6926 = t2067 * t127;
    let t6927 = t6926 * t616;
    let t6928 = t2034 * t6927;
    (t6916, t6917, t6919, t6922, t6923, t6925, t6926, t6927, t6928)
}
