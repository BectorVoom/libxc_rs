//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 667/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk667<F: Float>(t1894: F, t5447: F, t5448: F, t1647: F, t1898: F, t1907: F, t1945: F, t61: F, t1719: F, t695: F, t721: F, t1981: F, t1982: F, t4: F, t518: F, t615: F) -> (F, F, F, F, F, F) {
    let t5451 = 0.62071215503128080361e4 * t5447 * t1894 * t5448;
    let t5454 = 0.28947563097646563121e3 * t1907 * t1898 * t1647;
    let t5455 = t61 * t1945;
    let t5456 = t1719 * t695;
    let t5457 = t721 * t5456;
    let t5459 = 0.31168546390226634765e3 * t5455 * t5457;
    let t5460 = t61 * t1981;
    let t5461 = t1982 * t5456;
    let t5463 = 0.30762056574649219974e4 * t5460 * t5461;
    let t5464 = t4 * t518;
    let t5465 = t615 * t5464;
    (t5451, t5454, t5456, t5459, t5463, t5465)
}
