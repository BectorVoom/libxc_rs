//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 949/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk949<F: Float>(t3354: F, t4614: F, t597: F, t2437: F, t2877: F, t10309: F, t10313: F, t10317: F, t10321: F, t10323: F, t10326: F, t9266: F, t9270: F, t9276: F, t9281: F, t9289: F, t9296: F, t9307: F) -> (F, F) {
    let t10327 = t4614 * t3354;
    let t10329 = F::new(0.15337170381568299871e2) * t597 * t10327;
    let t10331 = F::new(0.35750489951850426669e0) * t2437 * t2877;
    let t10332 = -t9266 + t9270 - t9276 - t10309 - t10313 - t10317 - t10321 + t10323 - t9281 + t9289 + t9296 - t9307 - t10326 + t10329 + t10331;
    (t10327, t10332)
}
