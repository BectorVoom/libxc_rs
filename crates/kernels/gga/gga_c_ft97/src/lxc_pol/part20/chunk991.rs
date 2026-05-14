//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 991/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk991<F: Float>(t240: F, t43194: F, t10915: F, t13296: F, t13301: F, t13309: F, t13315: F, t13320: F, t13346: F, t13352: F, t13672: F, t14358: F, t1526: F, t15567: F, t17687: F, t17694: F, t2248: F, t231: F, t2917: F, t342: F, t343: F, t3691: F, t3700: F, t42264: F, t42267: F, t61123: F, t668: F, t69066: F, t69068: F, t69073: F, t69081: F, t703: F, t713: F) -> (F,) {
    let t69108 = t43194 * t240;
    let t69117 = t69066 - t69068 + 2.0 / 3.0 * t15567 * t17687 * t13352 + t69073 / 18.0 - t342 * t343 * t231 * t13672 / 4.0 - t69081 - t15567 * t17694 * t13346 / 2.0 + t1526 * t2248 * t703 * t240 * t668 / 6.0 + 2.0 * t14358 + t15567 * t17694 * t13296 / 6.0 - 2.0 / 3.0 * t61123 * t17694 * t13301 + t15567 * t2917 * t713 * t3700 / 3.0 - 2.0 / 9.0 * t15567 * t10915 * t713 * t3691 - t15567 * t17687 * t13309 / 9.0 - 7.0 / 27.0 * t15567 * t69108 * t13315 + 4.0 / 9.0 * t61123 * t17687 * t13320 + t42264 / 27.0 - t42267 / 18.0;
    (t69117,)
}
