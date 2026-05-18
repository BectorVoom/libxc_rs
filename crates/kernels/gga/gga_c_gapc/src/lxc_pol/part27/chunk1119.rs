//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 1119/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk1119<F: Float>(t17891: F, t17899: F, t26416: F, t291: F, t5542: F, t1089: F, t3687: F, t9906: F, t11945: F, t9895: F, t11878: F, t15805: F, t1936: F) -> (F, F, F, F) {
    let t33847 = t17891 * t5542 * t26416 * t291 * t17899;
    let t33850 = t9906 * t3687 * t1089;
    let t33852 = t9895 * t11945;
    let t33855 = t15805 * t1936 * t11878;
    (t33847, t33850, t33852, t33855)
}
