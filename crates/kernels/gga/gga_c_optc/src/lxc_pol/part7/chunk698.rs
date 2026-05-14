//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 698/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk698<F: Float>(t2093: F, t654: F, t141: F, t6560: F, t659: F, t2080: F, t661: F, t1948: F, t630: F, t629: F, t2089: F, t104: F, t137: F, t136: F, t6856: F, t131: F, t6165: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t6901 = t654 * t2093;
    let t6904 = t659 * t141 * t6560;
    let t6907 = t2080 * t661;
    let t6909 = t630 * t1948;
    let t6910 = t629 * t6909;
    let t6913 = t654 * t2089;
    let t6915 = t137 * t104;
    let t6916 = 1.0 / t6915;
    let t6917 = t136 * t6916;
    let t6919 = t6917 * t141 * t6856;
    let t6922 = t131 * t6165;
    (t6901, t6904, t6907, t6909, t6910, t6913, t6916, t6917, t6919, t6922)
}
