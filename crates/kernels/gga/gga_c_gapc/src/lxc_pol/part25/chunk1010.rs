//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1010/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1010<F: Float>(t1084: F, t291: F, t33521: F, t33527: F, t4052: F, t3095: F, t6182: F, t9438: F, t11808: F, t16181: F, t9863: F, t667: F, t8709: F, t17891: F, t29070: F, t1736: F, t188: F) -> (F, F, F, F, F, F, F) {
    let t33528 = t1084 * t4052 * t33521 * t291 * t33527;
    let t33530 = t3095 * t291;
    let t33532 = t9438 * t33530 * t6182;
    let t33536 = t11808 * t9863 * t16181;
    let t33539 = t667 * t8709 * M_PI;
    let t33541 = t17891 * t33539 * t29070;
    let t33543 = t188 * t1736;
    (t33528, t33530, t33532, t33536, t33539, t33541, t33543)
}
