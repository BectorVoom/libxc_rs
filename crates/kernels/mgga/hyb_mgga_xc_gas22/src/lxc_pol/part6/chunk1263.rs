//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1263/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1263<F: Float>(t1181: F, t7880: F, t3: F, t545: F, t2973: F, t10080: F, t125: F, t3916: F, t668: F, t10115: F, t1819: F, t555: F) -> (F, F, F, F, F, F) {
    let t27035 = t1181 * t7880;
    let t27037 = t545 * t3;
    let t27038 = t2973 * t27037;
    let t27066 = t10080 * t125;
    let t27071 = t3916 * t668;
    let t27085 = t555 * t1819 * t10115;
    (t27035, t27037, t27038, t27066, t27071, t27085)
}
