//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1010/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1010<F: Float>(t1815: F, t406: F, t1181: F, t599: F, t7413: F, t1859: F, t322: F, t604: F, t7493: F, t301: F, t8463: F, t6405: F, t7647: F, t5623: F, t7561: F, t5991: F, t7822: F) -> (F, F, F, F, F, F, F) {
    let t39794 = t1815 * t406;
    let t39797 = t7413 * t1181 * t599 * t39794;
    let t39802 = t7493 * t1181 * t604 * t1859 * t322;
    let t39807 = t8463 * t1181 * t604 * t1859 * t301;
    let t39809 = t7647 * t6405;
    let t39811 = t7561 * t5623;
    let t39813 = t7822 * t5991;
    (t39794, t39797, t39802, t39807, t39809, t39811, t39813)
}
