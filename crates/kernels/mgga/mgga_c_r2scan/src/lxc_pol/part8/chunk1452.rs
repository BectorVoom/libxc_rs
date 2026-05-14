//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1452/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1452<F: Float>(t322: F, t35071: F, t10469: F, t833: F, t1013: F, t10474: F, t10479: F, t1300: F, t19203: F, t2394: F, t2400: F, t2941: F, t2944: F, t31574: F, t31579: F, t327: F, t6692: F, t6693: F, t829: F, t834: F, t9676: F, t9679: F, t9684: F) -> (F,) {
    let t324 = 0.0 < t322;
    let t35072 = piecewise3(t324, 0.0, t35071);
    let t35075 = t10469 * t833;
    let t35109 = -0.64e0 * t35072 * t327 - 0.128e1 * t35075 * t829 - 0.384e1 * t31574 * t1013 - 0.1152e2 * t31579 * t2400 - 0.384e1 * t9679 * t2394 - 0.1152e2 * t9684 * t2394 - 0.1536e2 * t10474 * t6692 * t829 - 0.1536e2 * t19203 * t10474 * t829 - 0.1152e2 * t6693 * t2944 * t2394 - 0.1152e2 * t6693 * t10479 * t829 - 0.384e1 * t1300 * t2394 * t2941 - 0.384e1 * t1300 * t1013 * t9676 - 0.128e1 * t1300 * t10469 * t829 - 0.64e0 * t834 * t35072;
    (t35109,)
}
