//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 759/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk759<F: Float>(t1040: F, t8888: F, t1022: F, t1980: F, t5462: F, t641: F, t1: F, t1736: F, t102: F, t1648: F, t1894: F, t1026: F, t1846: F) -> (F, F, F, F, F, F) {
    let t8889 = t8888 * t1040;
    let t8891 = t1022 * t1980;
    let t8893 = t5462 * t641;
    let t8894 = t1736 * t1;
    let t8895 = t8894 * t102;
    let t8897 = t8895 * t1648 * t1894;
    let t8898 = t8893 * t8897;
    let t8900 = t1846 * t1026;
    (t8889, t8891, t8893, t8895, t8898, t8900)
}
