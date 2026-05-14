//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1145/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1145<F: Float>(t28128: F, t53798: F, t53891: F, t6161: F, t28361: F, t46862: F, t28405: F, t8392: F, t28401: F, t6914: F, t8232: F, t28388: F, t28346: F, t1443: F, t676: F, t28125: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t110669 = t53798 * t28128;
    let t110692 = t53891 * t6161;
    let t110702 = t46862 * t28361;
    let t110713 = 4.0 / 81.0 * t8392 * t28405;
    let t110718 = 4.0 / 27.0 * t8392 * t28401;
    let t110719 = t8232 * t6914;
    let t110733 = 4.0 / 27.0 * t8392 * t28388;
    let t110735 = 4.0 / 81.0 * t8392 * t28346;
    let t110751 = t676 * t1443;
    let t110796 = 2.0 / 27.0 * t8392 * t28125;
    (t110669, t110692, t110702, t110713, t110718, t110719, t110733, t110735, t110751, t110796)
}
