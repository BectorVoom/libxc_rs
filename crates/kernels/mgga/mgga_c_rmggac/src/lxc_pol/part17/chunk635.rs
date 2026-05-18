//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 635/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk635<F: Float>(t289: F, t8876: F, t2379: F, t4041: F, t2301: F, t2604: F, t1614: F, t645: F, t903: F, t2127: F, t534: F, t72: F) -> (F, F, F, F, F, F, F) {
    let t8877 = t289 * t8876;
    let t8879 = t4041 * t2379;
    let t8881 = t2604 * t2301;
    let t8884 = t645 * t1614;
    let t8885 = t903 * t8884;
    let t8887 = t534 * t2127;
    let t8888 = t72 * t8887;
    (t8877, t8879, t8881, t8884, t8885, t8887, t8888)
}
