//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1162/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1162<F: Float>(t100323: F, t22958: F, t5674: F, t100226: F, t22953: F, t100230: F, t25928: F, t100234: F, t23054: F, t25920: F, t25925: F, t25930: F, t25752: F, t45499: F, t2247: F, t22766: F, t25798: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t100466 = t5674 * t22958 * t100323;
    let t100469 = t5674 * t22953 * t100226;
    let t100472 = t5674 * t25928 * t100230;
    let t100475 = t5674 * t25928 * t100234;
    let t100477 = t23054 * t25920;
    let t100478 = 2.0 / 27.0 * t100477;
    let t100479 = t23054 * t25925;
    let t100480 = 2.0 / 27.0 * t100479;
    let t100481 = t23054 * t25930;
    let t100482 = 2.0 / 81.0 * t100481;
    let t100483 = t45499 * t25752;
    let t100491 = t22766 * t2247 * t25798;
    (t100466, t100469, t100472, t100475, t100477, t100478, t100479, t100480, t100481, t100482, t100483, t100491)
}
