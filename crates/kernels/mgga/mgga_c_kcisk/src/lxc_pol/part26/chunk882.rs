//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 882/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk882<F: Float>(t20233: F, t5676: F, t3935: F, t13607: F, t403: F, t5671: F, t2159: F, t3934: F, t394: F, t1224: F, t13524: F, t2075: F) -> (F, F, F, F, F) {
    let t20234 = t20233 * t5676;
    let t20236 = 0.2398771828823642295e-1 * t3935 * t20234;
    let t20237 = t13607 * t403;
    let t20238 = t20237 * t5671;
    let t20240 = 0.159918121921576153e-1 * t3935 * t20238;
    let t20255 = t2159 * t394 * t3934;
    let t20292 = t1224 * t13524 * t2075;
    (t20236, t20237, t20240, t20255, t20292)
}
