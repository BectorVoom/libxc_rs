//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1217/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1217<F: Float>(t1882: F, t30001: F, t29888: F, t103821: F, t103832: F, t103835: F, t103837: F, t103927: F, t11593: F, t116155: F, t11810: F, t1339: F, t15772: F, t16115: F, t16219: F, t16223: F, t16237: F, t1901: F, t23249: F, t26162: F, t26167: F, t26349: F, t26410: F, t3238: F, t446: F, t447: F, t452: F, t59631: F, t60901: F, t83: F) -> (F,) {
    let t118199 = t1882 * t30001;
    let t118201 = t1882 * t29888;
    let t118207 = -t446 * t447 * t1339 * t15772 / 9.0 + 2.0 / 3.0 * t446 * t452 * t3238 * t26410 - 10.0 / 81.0 * t1901 * t103927 * t16219 - 8.0 / 27.0 * t11593 * t26349 * t16223 - 4.0 / 3.0 * t1901 * t60901 * t26162 - 4.0 / 3.0 * t1901 * t59631 * t26167 - 4.0 / 27.0 * t103821 + 2.0 / 27.0 * t1901 * t26349 * t16237 - 8.0 / 27.0 * t103832 + t103835 - t103837 - t446 * t83 * t116155 / 3.0 + t118199 / 9.0 - 2.0 / 27.0 * t118201 + 4.0 / 3.0 * t1901 * t11810 * t23249 * t16115;
    (t118207,)
}
