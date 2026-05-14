//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 1066/1129 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk1066<F: Float>(t11303: F, t21842: F, t11500: F, t1717: F, t144: F, t21072: F, t21076: F, t26416: F, t5542: F, t3144: F, t34465: F, t11473: F, t3060: F, t3076: F, t11321: F, t5409: F) -> (F, F, F, F, F, F) {
    let t35203 = t11303 * t21842;
    let t35205 = t11500 * t1717;
    let t35210 = t21072 * t5542 * t26416 * t144 * t21076;
    let t35212 = t34465 * t3144;
    let t35215 = t3060 * t11473 * t3076;
    let t35217 = t11321 * t5409;
    (t35203, t35205, t35210, t35212, t35215, t35217)
}
