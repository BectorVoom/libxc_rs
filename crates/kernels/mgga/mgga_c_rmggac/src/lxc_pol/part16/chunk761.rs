//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 761/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk761<F: Float>(t1982: F, t7428: F, t8511: F, t16156: F, t9198: F, t388: F, t575: F, t7933: F, t7934: F, t535: F, t7244: F, t8422: F, t2310: F, t7939: F, t2283: F, t38354: F, t7473: F) -> (F, F, F, F, F, F, F, F) {
    let t41811 = t8511 * t7428 * t1982;
    let t41813 = t16156 * t9198;
    let t41817 = t7933 * t7934 * t388 * t575;
    let t41821 = t7933 * t7934 * t388 * t535;
    let t41828 = t7244 * t8422;
    let t41882 = t7939 * t2310;
    let t41884 = t7939 * t2283;
    let t41890 = t38354 * t7473;
    (t41811, t41813, t41817, t41821, t41828, t41882, t41884, t41890)
}
