//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 432/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk432<F: Float>(t2338: F, t356: F, t2164: F, t574: F, t1656: F, t640: F, t2402: F, t333: F, t1664: F, t668: F, t1614: F, t645: F) -> (F, F, F, F, F, F) {
    let t8854 = t2338 * t356;
    let t8858 = t2164 * t574;
    let t8862 = t640 * t1656;
    let t8866 = t2402 * t333;
    let t8876 = t1664 * t668;
    let t8884 = t645 * t1614;
    (t8854, t8858, t8862, t8866, t8876, t8884)
}
