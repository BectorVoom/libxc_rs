//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 854/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk854<F: Float>(t19034: F, t19306: F, t788: F, t1882: F, t5332: F, t5323: F, t5319: F, t1212: F, t4299: F, t840: F, t871: F, t4246: F, t296: F, t5374: F, t870: F, t875: F) -> (F, F, F, F, F, F, F, F) {
    let t19307 = t19034 + t19306;
    let t19308 = t788 * t19307;
    let t19318 = t1882 * t5332;
    let t19320 = t1882 * t5323;
    let t19322 = t1882 * t5319;
    let t19324 = t1212 * t4299;
    let t19326 = t840 * t871 * t19324;
    let t19329 = t4246 * t4299;
    let t19330 = t296 * t19329;
    let t19333 = t5374 * t870;
    let t19334 = t19333 * t875;
    (t19308, t19318, t19320, t19322, t19326, t19329, t19330, t19334)
}
