//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1535/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1535<F: Float>(t2857: F, t3154: F, t2251: F, t2258: F, t10326: F, t606: F, t11262: F, t3127: F, t3129: F, t11240: F, t11628: F, t42646: F) -> (F, F, F, F, F) {
    let t43174 = t3154 * t2857;
    let t43175 = t43174 * t2251;
    let t43180 = t2251 * t2258;
    let t43194 = t10326 * t606;
    let t43204 = t3127 * t11262 * t3129;
    let t43207 = t11240 * t11628 * t42646;
    (t43175, t43180, t43194, t43204, t43207)
}
