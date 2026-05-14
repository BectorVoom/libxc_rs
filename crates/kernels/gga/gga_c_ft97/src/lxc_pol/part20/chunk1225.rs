//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1225/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1225<F: Float>(t43917: F, t6318: F, t113190: F, t14686: F, t28731: F, t99312: F, t28506: F, t684: F, t24976: F, t6317: F, t28726: F, t24980: F, t24981: F, t2789: F, t6334: F, t992: F) -> (F, F, F, F, F, F, F, F) {
    let t113191 = t43917 * t6318;
    let t113193 = t113190 * t113191 * t14686;
    let t113195 = t99312 * t28731;
    let t113196 = t113195 / 18.0;
    let t113197 = t28506 * t684;
    let t113199 = t6317 * t24976 * t113197;
    let t113201 = t99312 * t28726;
    let t113202 = t113201 / 18.0;
    let t113206 = t24980 * t24981 * t6334 * t992 * t2789;
    (t113193, t113195, t113196, t113197, t113199, t113201, t113202, t113206)
}
