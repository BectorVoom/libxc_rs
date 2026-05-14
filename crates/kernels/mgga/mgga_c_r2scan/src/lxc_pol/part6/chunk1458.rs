//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1458/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1458<F: Float>(t1044: F, t1353: F, t19611: F, t19614: F, t19620: F, t19624: F, t19628: F, t19720: F, t23935: F, t23937: F, t23938: F, t2881: F, t6772: F, t19646: F, t19649: F, t19687: F, t19728: F, t23943: F, t23949: F, t23951: F, t23954: F, t23956: F, t23959: F, t23961: F) -> (F, F) {
    let t27446 = t1044 * t6772 + 3.0 * t1353 * t2881 + t19611 + t19614 - t19620 + t19624 - t19628 + t19720 + t23935 - t23937 - t23938;
    let t27449 = -t19646 - t19649 - t19728 + t23943 + t23949 + t19687 + t23951 - t23954 + t23956 - t23959 - t23961;
    (t27446, t27449)
}
