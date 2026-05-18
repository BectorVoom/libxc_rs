//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 272/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk272<F: Float>(t63: F, t66: F, t77: F, t929: F, t401: F, t930: F, t6: F, t1620: F, t428: F, t374: F, t383: F, t1631: F) -> (F, F, F, F, F, F, F, F) {
    let t3020 = t63 * t66;
    let t3021 = t77 * t929;
    let t3022 = t3020 * t3021;
    let t3025 = t930 * t401;
    let t3029 = t930 * t6;
    let t3030 = t3029 * t1620;
    let t3033 = t930 * t428;
    let t3034 = t374 * t3033;
    let t3037 = t930 * t383;
    let t3038 = t1631 * t3037;
    (t3021, t3022, t3025, t3030, t3033, t3034, t3037, t3038)
}
