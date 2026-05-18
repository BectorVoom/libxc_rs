//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 392/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk392<F: Float>(t1900: F, t1903: F, t1743: F, t198: F, t199: F) -> (F, F, F, F, F) {
    let t1904 = t1900 * t1903;
    let t1905 = t1743 * t1904;
    let t1906 = M_PI * t198;
    let t1907 = t199 * t199;
    let t1908 = F::new(1.0) / t1907;
    (t1904, t1905, t1906, t1907, t1908)
}
