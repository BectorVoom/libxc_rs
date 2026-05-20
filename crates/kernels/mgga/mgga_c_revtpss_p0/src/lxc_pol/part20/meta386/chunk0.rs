//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1416/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1416<F: Float>(t11341: F, t141: F, t41248: F, t10326: F, t2857: F, t606: F, t930: F, t2852: F, t2908: F, t11150: F, t2251: F, t2258: F) -> (F, F, F, F, F, F) {
    let t41250 = t141 * t11341 * t41248;
    let t41253 = t2857 * t10326 * t606;
    let t41255 = t141 * t930 * t41253;
    let t41258 = t2852 * t10326 * t606;
    let t41260 = t141 * t2908 * t41258;
    let t41263 = t11150 * t2251 * t2258;
    (t41250, t41253, t41255, t41258, t41260, t41263)
}
