//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 720/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk720<F: Float>(t16891: F, t4699: F, t1014: F, t4674: F, t12401: F, t4702: F, t1013: F, t16907: F, t3355: F, t4710: F, t19977: F, t8690: F) -> (F, F, F, F, F, F) {
    let t20576 = t16891 * t4699;
    let t20578 = t4674 * t1014;
    let t20580 = t12401 * t4702;
    let t20583 = t16907 * t1013;
    let t20586 = t3355 * t4710;
    let t20589 = t8690 * t19977;
    (t20576, t20578, t20580, t20583, t20586, t20589)
}
