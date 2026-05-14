//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1202/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1202<F: Float>(t5059: F, t24357: F, t277: F, t33574: F, t33596: F, t52260: F, t52264: F, t57251: F, t57253: F, t57257: F, t57260: F, t57520: F, t57523: F, t57525: F, t95: F, t43347: F, t52269: F, t57327: F, t57330: F, t57332: F, t57335: F, t57337: F, t57343: F, t57346: F, t57349: F, t57351: F, t57527: F) -> (F, F) {
    let t58229 = t5059 * t5059;
    let t58237 = 20.0 / 81.0 * t33574 - t57251 + t57253 + t57257 - 0.15506928860942058298e-1 * t95 * t277 * t58229 * t24357 + t57260 + 20.0 / 27.0 * t33596 + t57520 - t57523 - t57525 + 56.0 / 81.0 * t52260 + 8.0 / 9.0 * t52264;
    let t58240 = 2.0 / 9.0 * t52269 - t57327 + 4.0 / 9.0 * t43347 - t57330 - t57332 - t57335 + t57337 - t57343 + t57346 + t57349 - t57351 - t57527;
    (t58237, t58240)
}
