//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 999/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk999<F: Float>(t14545: F, t213: F, t2777: F, t4518: F, t2439: F, t2470: F, t4499: F, t2798: F, t1568: F, t2783: F, t786: F, t2801: F, t233: F, t4469: F, t869: F, t689: F) -> (F, F, F, F, F, F) {
    let t14546 = t213 * t14545;
    let t14557 = t2777 * t4518;
    let t14558 = t2439 * t14557;
    let t14563 = t4499 * t2470;
    let t14564 = t2798 * t14563;
    let t14567 = t2783 * t1568;
    let t14568 = t786 * t14567;
    let t14570 = 0.19514881078765566038e-1 * t14568 * t2801;
    let t14574 = t233 * t4469;
    let t14575 = t869 * t14574;
    let t14577 = 0.10975748638225852664e-1 * t689 * t14575;
    (t14546, t14558, t14564, t14568, t14570, t14577)
}
