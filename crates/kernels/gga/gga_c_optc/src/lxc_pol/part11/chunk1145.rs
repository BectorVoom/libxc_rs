//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1145/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1145<F: Float>(t13602: F, t4054: F, t16886: F, t2367: F, t999: F, t10838: F, t16636: F, t862: F, t17035: F, t24503: F, t17056: F, t24: F) -> (F, F, F, F, F) {
    let t49803 = t4054 * t13602;
    let t49808 = t999 * t2367 * t16886;
    let t49816 = t862 * t10838 * t16636;
    let t49822 = t24503 * t17035;
    let t49833 = t862 * t24 * t17056;
    (t49803, t49808, t49816, t49822, t49833)
}
