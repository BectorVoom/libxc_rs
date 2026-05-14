//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 881/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk881<F: Float>(t10314: F, t6964: F, t6963: F, t2299: F, t986: F, t1415: F, t1646: F, t2877: F, t9285: F, t3390: F, t4614: F, t574: F, t3354: F, t597: F, t2437: F, t10309: F, t10313: F, t9266: F, t9270: F, t9276: F, t9281: F, t9289: F, t9296: F, t9307: F) -> (F, F, F, F, F, F) {
    let t10315 = t6964 * t10314;
    let t10317 = 0.71500979903700853338e0 * t6963 * t10315;
    let t10318 = t2299 * t986;
    let t10319 = t1415 * t10318;
    let t10321 = 0.35750489951850426669e0 * t10319 * t1646;
    let t10323 = 0.35750489951850426669e0 * t9285 * t2877;
    let t10324 = t4614 * t3390;
    let t10326 = 0.61348681526273199483e1 * t574 * t10324;
    let t10327 = t4614 * t3354;
    let t10329 = 0.15337170381568299871e2 * t597 * t10327;
    let t10331 = 0.35750489951850426669e0 * t2437 * t2877;
    let t10332 = -t9266 + t9270 - t9276 - t10309 - t10313 - t10317 - t10321 + t10323 - t9281 + t9289 + t9296 - t9307 - t10326 + t10329 + t10331;
    (t10315, t10318, t10319, t10324, t10327, t10332)
}
