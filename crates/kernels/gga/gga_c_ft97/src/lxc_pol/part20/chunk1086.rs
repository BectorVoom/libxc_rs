//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1086/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1086<F: Float>(t172: F, t6817: F, t231: F, t27616: F, t27620: F, t3794: F, t70: F, t6034: F, t6037: F, t65692: F, t695: F, t24305: F, t27671: F, t35410: F, t2380: F, t66422: F) -> (F, F, F, F, F, F) {
    let t108572 = t6817 * t172;
    let t108573 = t108572 * t231;
    let t108576 = 0.3520097786805302698e-5 * t27616 * t108573 * t27620;
    let t108581 = t3794 * t70;
    let t108583 = t6034 * t108581 * t6037;
    let t108585 = t65692 * t695;
    let t108586 = t24305 * t108585;
    let t108587 = t27671 * t35410;
    let t108590 = t66422 * t2380;
    (t108572, t108576, t108583, t108586, t108587, t108590)
}
