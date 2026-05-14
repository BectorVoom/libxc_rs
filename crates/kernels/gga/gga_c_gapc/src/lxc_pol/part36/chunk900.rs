//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 900/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk900<F: Float>(t2315: F, t2553: F, t6: F, t7592: F, t7593: F, t2776: F, t286: F, t8132: F, t7591: F, t8141: F, t952: F, t291: F, t4043: F, t959: F, t1153: F, t2417: F) -> (F, F, F, F, F, F, F) {
    let t16133 = t2553 * t2315;
    let t16152 = t7592 * t7593 * t6;
    let t16181 = t2776 * t286;
    let t16182 = t8132 * t16181;
    let t16296 = t7591 * t952 * t8141;
    let t16403 = t4043 * t291 * t959;
    let t16404 = t2417 * t1153;
    (t16133, t16152, t16181, t16182, t16296, t16403, t16404)
}
