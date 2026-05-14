//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1450/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1450<F: Float>(t1010: F, t10451: F, t10454: F, t10466: F, t1271: F, t1276: F, t19146: F, t19155: F, t19182: F, t2378: F, t2391: F, t2928: F, t2938: F, t313: F, t35065: F, t6654: F, t6661: F, t819: F, t826: F, t9673: F) -> (F,) {
    let t35070 = -3.0 * t2378 * t9673 - 6.0 * t19146 * t10451 + 24.0 * t19155 * t10451 * t826 - 18.0 * t6661 * t2928 * t2391 + 6.0 * t6654 * t10454 - 18.0 * t6661 * t10454 * t826 + 6.0 * t1276 * t2391 * t2938 + 6.0 * t1276 * t1010 * t9673 - t1271 * t10466 + 2.0 * t1276 * t10466 * t826 - t819 * (3.0 / 10.0 * t313 * t35065 + t19182);
    (t35070,)
}
