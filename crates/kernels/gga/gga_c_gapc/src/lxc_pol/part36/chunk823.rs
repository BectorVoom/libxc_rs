//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 823/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk823<F: Float>(t9843: F, t9846: F, t7259: F, t9067: F, t8142: F, t1084: F, t9282: F, t3415: F, t2619: F, t9083: F, t7939: F, t8769: F) -> (F, F, F, F, F) {
    let t9847 = t9843 * t9846;
    let t9849 = t7259 * t9067;
    let t9850 = t9849 * t8142;
    let t9852 = t1084 * t9282;
    let t9853 = t9852 * t3415;
    let t9856 = t2619 * t9083;
    let t9857 = t9856 * t7939;
    let t9859 = t2619 * t8769;
    (t9847, t9850, t9853, t9857, t9859)
}
