//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 848/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk848<F: Float>(t9862: F, t9865: F, t197: F, t7460: F, t1077: F, t7843: F, t3336: F, t1081: F, t2737: F, t3418: F, t7511: F, t3421: F) -> (F, F, F, F, F, F) {
    let t9866 = t9862 * t9865;
    let t9868 = t197 * t7460;
    let t9869 = t1077 * t9868;
    let t9871 = t197 * t7843;
    let t9872 = t3336 * t9871;
    let t9874 = t1081 * t2737;
    let t9876 = t3418 * t7511;
    let t9878 = t3421 * t7511;
    (t9866, t9869, t9872, t9874, t9876, t9878)
}
