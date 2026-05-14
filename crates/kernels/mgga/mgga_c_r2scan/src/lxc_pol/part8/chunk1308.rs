//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1308/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1308<F: Float>(t23709: F, t481: F, t986: F, t2266: F, t8601: F, t18786: F, t18839: F, t18843: F, t18855: F, t19439: F, t23320: F, t23321: F, t23689: F, t23694: F, t31357: F, t32071: F, t32075: F) -> (F, F, F) {
    let t32093 = 0.14447919941302971323e1 * t23709;
    let t32094 = t986 * t481;
    let t32097 = 9.0 * t2266 * t8601 * t32094;
    let t32102 = t18786 - t32071 + t23320 + t23321 + t18839 - t18843 + t19439 - 0.7089e1 * t31357 + t32075 - 0.7089e1 * t23689 + t18855 + t23694;
    (t32093, t32097, t32102)
}
