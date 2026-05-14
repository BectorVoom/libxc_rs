//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 717/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk717<F: Float>(t647: F, t9306: F, t2998: F, t9305: F, t2993: F, t9282: F, t3001: F, t129: F, t5987: F, t2987: F, t197: F, t5799: F, t2986: F, t2003: F, t3039: F, t144: F, t1736: F) -> (F, F, F, F, F, F, F) {
    let t9307 = t647 * t9306;
    let t9308 = t2998 * t9307;
    let t9309 = t9305 * t9308;
    let t9311 = t2993 * t9282;
    let t9312 = t9311 * t3001;
    let t9314 = t5987 * t129;
    let t9315 = t9314 * t2987;
    let t9317 = t197 * t5799;
    let t9318 = t2986 * t9317;
    let t9320 = t3039 * t2003;
    let t9323 = t1736 * t144;
    (t9308, t9309, t9312, t9315, t9318, t9320, t9323)
}
