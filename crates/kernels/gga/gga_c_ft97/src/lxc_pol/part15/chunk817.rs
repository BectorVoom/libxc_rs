//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 817/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk817<F: Float>(t1614: F, t2426: F, t3771: F, t679: F, t4977: F, t694: F, t3724: F, t5049: F, t237: F, t2382: F, t4985: F, t1771: F, t4966: F, t4974: F, t4970: F, t13467: F, t3758: F) -> (F, F, F, F, F, F, F, F) {
    let t66096 = t2426 * t1614;
    let t66098 = t3771 * t66096 * t679;
    let t66115 = t694 * t4977;
    let t66137 = t3724 * t694 * t5049;
    let t66154 = t2382 * t4985 * t237;
    let t66197 = t1771 * t4966;
    let t66202 = t1771 * t4974;
    let t66221 = t1771 * t4970;
    let t66313 = t3758 * t13467;
    (t66098, t66115, t66137, t66154, t66197, t66202, t66221, t66313)
}
