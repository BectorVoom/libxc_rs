//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1384/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1384<F: Float>(t1882: F, t27269: F, t26969: F, t26982: F, t8392: F, t105336: F, t106167: F, t106197: F, t11437: F, t12599: F, t12709: F, t12747: F, t12945: F, t13204: F, t13216: F, t1391: F, t143: F, t144: F, t160: F, t1901: F, t2185: F, t26863: F, t27220: F, t27334: F, t27335: F, t28: F, t3052: F, t3281: F, t3430: F, t446: F, t51036: F, t569: F, t5856: F, t5975: F, t89: F, t95767: F, t95789: F) -> (F,) {
    let t107336 = 4.0 / 9.0 * t1882 * t27269;
    let t107361 = 2.0 / 9.0 * t1882 * t26969;
    let t107370 = 4.0 / 9.0 * t8392 * t26982;
    let t107377 = 4.0 / 9.0 * t1901 * t26863 * t12747 + 2.0 / 3.0 * t446 * t144 * t105336 - t107336 - 4.0 / 9.0 * t3281 * t569 * t5975 * t3052 + 2.0 / 3.0 * t446 * t2185 * t1391 * t12945 + t89 * t28 * t143 * t106167 * t160 / 3.0 + 4.0 / 3.0 * t446 * t144 * t106197 + 2.0 / 3.0 * t1901 * t12709 * t27220 * t11437 + 2.0 / 9.0 * t1901 * t95767 * t3430 + t107361 + 2.0 / 9.0 * t1901 * t51036 * t5856 - 2.0 * t1901 * t27334 * t27335 * t12599 + t107370 - 2.0 / 9.0 * t1901 * t95789 * t13204 - 2.0 / 9.0 * t1901 * t95789 * t13216;
    (t107377,)
}
