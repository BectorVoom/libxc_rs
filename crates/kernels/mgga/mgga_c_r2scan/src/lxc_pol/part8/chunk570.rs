//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 570/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk570<F: Float>(t322: F, t1010: F, t1271: F, t1276: F, t2376: F, t2378: F, t2381: F, t2391: F, t321: F, t819: F, t826: F, t1013: F, t833: F, t829: F, t1300: F, t327: F, t834: F) -> (F, F, F, F, F) {
    let t324 = 0.0 < t322;
    let t2393 = -t1271 * t1010 + 2.0 * t1276 * t2381 + t2376 * t321 - t2378 * t826 - t819 * t2391;
    let t2394 = piecewise3(t324, 0.0, t2393);
    let t2397 = t1013 * t833;
    let t2400 = t1013 * t829;
    let t2405 = -0.64e0 * t2394 * t327 - 0.128e1 * t2397 * t829 - 0.128e1 * t1300 * t2400 - 0.64e0 * t834 * t2394;
    (t2393, t2394, t2397, t2400, t2405)
}
