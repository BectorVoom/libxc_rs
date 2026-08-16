//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 399/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk399<F: Float>(t1969: F, t379: F, t5900: F, t5899: F, t2112: F, t5860: F, t1369: F, t28: F, t5842: F, t586: F, t1374: F, t375: F, t89: F) -> (F, F, F, F, F, F, F) {
    let t5902 = t1969 * t5900 * t379;
    let t5903 = t5899 * t5902;
    let t5905 = t2112 * t5860;
    let t5907 = t1369 * t28 * t5905;
    let t5909 = t586 * t5842;
    let t5911 = t1369 * t28 * t5909;
    let t5914 = t89 * t375 * t1374;
    (t5902, t5903, t5905, t5907, t5909, t5911, t5914)
}
