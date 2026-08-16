//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 644/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk644<F: Float>(t11997: F, t11998: F, t12011: F, t12028: F, t3699: F, t501: F, t3718: F, t605: F, t12007: F, t549: F, t10309: F, t10313: F, t10317: F, t10321: F, t10323: F, t10326: F, t10329: F, t1429: F, t9265: F, t9270: F, t9276: F, t9280: F, t9289: F, t9296: F, t9307: F) -> (F, F, F, F) {
    let t12030 = t11997 + t11998 + t12011 + t12028;
    let t12032 = t3699 * t501;
    let t12035 = t3718 * t605;
    let t12038 = t549 * t12007;
    let t12043 = F::cast_from(0.39722766613167140743e-1_f64) * t1429 * t12038 - F::cast_from(0.76685851907841499354e0_f64) * t9265 + t9270 - t9276 - t10309 - t10313 - t10317 - t10321 + t10323 - F::cast_from(0.38342925953920749677e0_f64) * t9280 + t9289 + t9296 - t9307 - t10326 + t10329;
    (t12030, t12032, t12035, t12043)
}
