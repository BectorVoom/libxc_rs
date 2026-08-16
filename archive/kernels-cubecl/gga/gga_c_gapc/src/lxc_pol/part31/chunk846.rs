//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 846/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk846<F: Float>(t2300: F, t2636: F, t3396: F, t2979: F, t7624: F, t2255: F, t2982: F, t2619: F, t9128: F, t3388: F, t916: F, t3392: F) -> (F, F, F, F, F) {
    let t9620 = t2636 * t2300;
    let t9621 = t3396 * t9620;
    let t9623 = t7624 * t2979;
    let t9624 = t2982 * t2255;
    let t9625 = t9623 * t9624;
    let t9627 = t2619 * t9128;
    let t9628 = t9627 * t3388;
    let t9630 = t916 * t9128;
    let t9631 = t9630 * t3392;
    (t9621, t9624, t9625, t9628, t9631)
}
