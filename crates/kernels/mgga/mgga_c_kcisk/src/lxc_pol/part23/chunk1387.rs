//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1387/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1387<F: Float>(t13329: F, t394: F, t1411: F, t2262: F, t3786: F, t1299: F, t33609: F, t3784: F, t1333: F, t33498: F, t3777: F, t51854: F, t9461: F, t33605: F, t3748: F, t2270: F) -> (F, F, F, F, F, F, F) {
    let t114585 = t13329 * t394;
    let t114588 = t1411 * t114585 * t2262 * t3786;
    let t114592 = t1411 * t3784 * t1299 * t33609;
    let t114596 = t1333 * t33498;
    let t114597 = 0.33163888888888888888e-2 * t114596;
    let t114604 = t1411 * t9461 * t51854 * t3777;
    let t114606 = t3748 * t33605;
    let t114608 = t2270 * t1299;
    (t114588, t114592, t114596, t114597, t114604, t114606, t114608)
}
