//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 769/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk769<F: Float>(t1051: F, t2046: F, t3201: F, t731: F, t2155: F, t2674: F, t825: F, t996: F, t2255: F, t493: F, t1063: F, t2221: F, t2229: F, t2453: F, t3259: F, t2436: F, t3234: F) -> (F, F, F, F, F, F, F, F) {
    let t10174 = t2046 * t1051;
    let t10176 = t731 * t3201;
    let t10178 = t2155 * t1051;
    let t10180 = t2674 * t825;
    let t10181 = t996 * t10180;
    let t10182 = t493 * t2255;
    let t10183 = t10181 * t10182;
    let t10185 = t2221 * t1063;
    let t10187 = t2229 * t1063;
    let t10189 = t2453 * t3259;
    let t10191 = t3234 * t2436;
    (t10174, t10176, t10178, t10183, t10185, t10187, t10189, t10191)
}
