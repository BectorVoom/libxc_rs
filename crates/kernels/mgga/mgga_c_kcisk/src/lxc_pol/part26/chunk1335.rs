//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1335/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1335<F: Float>(t1339: F, t25392: F, t32045: F, t109420: F, t8171: F, t109883: F, t26940: F, t3482: F, t415: F, t8162: F, t9474: F, t33377: F, t33451: F, t113941: F, t114405: F, t119019: F, t119372: F, t1220: F, t1299: F, t20: F, t2718: F, t32087: F, t33346: F, t33373: F, t33384: F, t33417: F, t33434: F, t33439: F, t33477: F, t8020: F, t9446: F) -> (F, F, F, F, F) {
    let t119385 = t1339 * t32045 * t25392;
    let t119388 = t1339 * t109420 * t8171;
    let t119399 = t3482 * t109883 * t26940;
    let t119402 = t415 * t8162 * t9474;
    let t119404 = t33377 * t33451;
    let t119413 = -0.92592592592592592594e-2 * t113941 * t33417 + 0.69444444444444444446e-2 * t32087 * t119372 - 0.33163888888888888888e-2 * t119385 - 0.33163888888888888888e-2 * t119388 - 0.13888888888888888889e-1 * t33373 * t33477 - 0.41666666666666666668e-1 * t33384 * t33434 - 0.20833333333333333334e-1 * t33384 * t33439 - 0.41666666666666666668e-1 * t9446 * t119019 - 0.22109259259259259258e-2 * t119399 - 0.66327777777777777776e-2 * t119402 + 0.26805555555555555557e-2 * t119404 + t114405 + 0.20833333333333333334e-1 * t33373 * t33346 + 0.27777777777777777779e-1 * t1220 * t8020 * t1299 * t20 * t2718;
    (t119385, t119388, t119399, t119402, t119413)
}
