//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 801/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk801<F: Float>(t2586: F, t5007: F, t940: F, t5002: F, t888: F, t2758: F, t4997: F, t2751: F, t2367: F, t4937: F, t930: F, t14289: F, t953: F) -> (F, F, F, F, F) {
    let t14762 = t2586 * t5007;
    let t14763 = t940 * t14762;
    let t14766 = t888 * t5002;
    let t14767 = t2758 * t14766;
    let t14773 = t888 * t4997;
    let t14774 = t2751 * t14773;
    let t14778 = t2367 * t4937;
    let t14779 = t930 * t14778;
    let t14783 = t953 * t14289;
    (t14763, t14767, t14774, t14779, t14783)
}
