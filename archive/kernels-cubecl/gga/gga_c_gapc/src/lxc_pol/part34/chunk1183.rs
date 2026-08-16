//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1183/1427 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1183<F: Float>(t11964: F, t26447: F, t29314: F, t311: F, t1971: F, t9244: F, t1084: F, t9929: F, t11910: F, t30095: F, t2562: F, t7120: F) -> (F, F, F, F, F) {
    let t33893 = t311 * t11964 * t26447 * t29314;
    let t33895 = t1971 * t9244;
    let t33897 = t1084 * t33895 * t9929;
    let t33899 = t11910 * t30095;
    let t33901 = t7120 * t2562;
    (t33893, t33895, t33897, t33899, t33901)
}
