//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 919/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk919<F: Float>(t1771: F, t4974: F, t4970: F, t13467: F, t3758: F, t17957: F, t2426: F, t3724: F, t5025: F, t236: F, t5005: F, t13580: F) -> (F, F, F, F, F, F) {
    let t66202 = t1771 * t4974;
    let t66221 = t1771 * t4970;
    let t66313 = t3758 * t13467;
    let t66318 = t3758 * t17957;
    let t66328 = t3724 * t2426 * t5025;
    let t66354 = t236 * t5005;
    let t66355 = t13580 * t66354;
    (t66202, t66221, t66313, t66318, t66328, t66355)
}
