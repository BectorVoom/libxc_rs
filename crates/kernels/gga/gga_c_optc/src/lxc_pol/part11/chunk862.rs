//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 862/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk862<F: Float>(t10188: F, t13699: F, t13701: F, t13703: F, t16630: F, t16634: F, t16638: F, t16642: F, t16646: F, t16756: F, t16759: F, t17299: F, t10760: F, t13733: F, t1426: F, t17276: F, t17284: F, t17287: F, t2301: F, t350: F, t4009: F, t4835: F, t4846: F, t8345: F, t974: F) -> (F, F) {
    let t17311 = 0.48461111111111111112e3 * t13699 - 0.14538333333333333333e4 * t13701 + 0.72691666666666666668e3 * t13703 - 0.96922222222222222223e3 * t10188 + 0.29076666666666666666e4 * t16634 - 0.14538333333333333333e4 * t16638 - 0.43614999999999999999e4 * t16642 + 0.43614999999999999999e4 * t16646 - 0.80768518518518518518e3 * t16630 - 0.34962962962962962963e2 * t16756 - 0.78666666666666666667e2 * t16759;
    let t17312 = t17299 + t17311;
    let t17314 = 6.0 * t10760 * t4835 - 3.0 * t13733 * t1426 + t17276 * t350 - 6.0 * t8345 * t17284 + 6.0 * t2301 * t17287 - t974 * t17312 - 3.0 * t4009 * t4846;
    (t17312, t17314)
}
