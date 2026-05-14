//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 863/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk863<F: Float>(t1160: F, t3065: F, t955: F, t3073: F, t945: F, t1237: F, t13259: F, t3066: F, t3077: F, t3088: F, t3089: F, t13326: F, t183: F, t14402: F, t453: F, t1035: F, t1240: F, t3044: F) -> (F, F, F, F, F, F, F, F) {
    let t14528 = t1160 * t3065 * t955;
    let t14534 = t3073 * t3065 * t945;
    let t14539 = t13259 * t1237;
    let t14547 = t3077 * t3066;
    let t14551 = t3088 * t3089 * t955;
    let t14554 = 0.65854491829355115987e0 * t13326 * t183;
    let t14556 = 0.26341796731742046395e1 * t14402 * t453;
    let t14564 = t1035 * t1240 * t3044;
    (t14528, t14534, t14539, t14547, t14551, t14554, t14556, t14564)
}
