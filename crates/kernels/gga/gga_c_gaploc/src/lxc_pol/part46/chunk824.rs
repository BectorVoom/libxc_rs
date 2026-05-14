//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 824/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk824<F: Float>(t40898: F, t40900: F, t13176: F, t731: F, t22090: F, t2508: F, t28668: F, t8604: F, t11004: F, t7226: F, t7291: F, t40902: F, t10789: F, t7667: F, t13188: F, t7137: F) -> (F, F, F, F, F, F, F, F) {
    let t43288 = 0.85450291446024714264e-3 * t40898;
    let t43289 = 0.85450291446024714264e-3 * t40900;
    let t43290 = t731 * t13176;
    let t43295 = 0.1845726295234133828e0 * t2508 * t22090 * t8604 * t28668;
    let t43298 = t2508 * t7226 * t11004 * t7291;
    let t43300 = 0.64087718584518535698e-3 * t40902;
    let t43302 = t2508 * t10789 * t7667;
    let t43304 = t7137 * t13188;
    (t43288, t43289, t43290, t43295, t43298, t43300, t43302, t43304)
}
