//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 622/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk622<F: Float>(t1154: F, t2476: F, t91: F, t9890: F, t2475: F, t3938: F, t747: F, t13378: F, t2354: F, t446: F, t13383: F, t9744: F, t1882: F, t3714: F, t13390: F, t13292: F) -> (F, F, F, F, F, F, F, F) {
    let t13768 = t91 * t9890 * t1154 * t2476;
    let t13770 = t2475 * t3938;
    let t13772 = t91 * t13770 * t747;
    let t13774 = t2354 * t13378;
    let t13775 = t446 * t13774;
    let t13777 = t9744 * t13383;
    let t13778 = t446 * t13777;
    let t13780 = t1882 * t3714;
    let t13781 = 2.0 / 27.0 * t13780;
    let t13782 = t2354 * t13390;
    let t13783 = t446 * t13782;
    let t13785 = t2354 * t13292;
    (t13768, t13772, t13775, t13778, t13780, t13781, t13783, t13785)
}
