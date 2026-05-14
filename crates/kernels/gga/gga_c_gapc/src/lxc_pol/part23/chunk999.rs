//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 999/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk999<F: Float>(t11902: F, t16296: F, t11905: F, t18018: F, t1086: F, t11990: F, t15835: F, t11741: F, t16133: F, t3284: F, t1734: F, t24759: F, t1084: F, t29654: F, t15680: F, t26312: F, t3402: F) -> (F, F, F, F, F, F, F) {
    let t34176 = t11902 * t16296;
    let t34178 = t11905 * t18018;
    let t34181 = t11990 * t1086 * t15835;
    let t34184 = t11741 * t3284 * t16133;
    let t34186 = t1734 * t24759;
    let t34188 = t1084 * t34186 * t29654;
    let t34191 = t3402 * t26312 * t15680;
    (t34176, t34178, t34181, t34184, t34186, t34188, t34191)
}
