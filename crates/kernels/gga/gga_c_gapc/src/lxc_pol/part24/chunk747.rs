//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 747/1327 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk747<F: Float>(t505: F, t674: F, t3143: F, t3139: F, t3060: F, t3120: F, t1040: F, t1022: F, t1980: F, t5462: F, t641: F, t1: F, t1736: F) -> (F, F, F, F, F, F, F, F) {
    let t8884 = M_PI * t505 * t674;
    let t8885 = t8884 * t3143;
    let t8886 = t3139 * t8885;
    let t8888 = t3060 * t3120;
    let t8889 = t8888 * t1040;
    let t8891 = t1022 * t1980;
    let t8893 = t5462 * t641;
    let t8894 = t1736 * t1;
    (t8884, t8885, t8886, t8888, t8889, t8891, t8893, t8894)
}
