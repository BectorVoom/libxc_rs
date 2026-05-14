//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1434/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1434<F: Float>(t761: F, t7760: F, t2061: F, t19687: F, t19728: F, t19748: F, t22651: F, t23951: F, t23954: F, t23956: F, t23959: F, t23961: F, t23970: F, t23972: F, t19694: F, t19698: F, t19702: F, t22661: F, t23980: F, t23982: F, t23984: F, t23986: F, t23992: F, t25032: F, t25034: F) -> (F, F) {
    let t26976 = t7760 * t761;
    let t26977 = t2061 * t26976;
    let t26980 = t19728 - t19687 - t23951 + t23954 - t23956 + t23959 + t23961 - 0.4051561992e0 * t26977 - t23970 - 0.12154685976e1 * t22651 + t23972 + t19748;
    let t26983 = t23980 - t23982 + t23984 + t19694 - t19698 - t23986 - t19702 + t23992 + t25032 - t25034 - 0.5143752e0 * t22661;
    (t26980, t26983)
}
