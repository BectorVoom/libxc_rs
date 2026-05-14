//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1393/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1393<F: Float>(t19694: F, t19698: F, t19702: F, t19748: F, t22651: F, t23956: F, t23961: F, t23982: F, t23986: F, t32991: F, t32992: F, t32994: F, t19709: F, t22661: F, t22665: F, t22669: F, t22674: F, t25032: F, t29028: F, t29030: F, t32995: F, t32996: F, t32997: F) -> (F, F) {
    let t33785 = t32991 - t23956 - t23961 - 0.4051561992e0 * t22651 + t32992 + t19748 - t23982 + t19694 - t19698 + t23986 - t19702 + t32994;
    let t33790 = t25032 - 0.1714584e0 * t22661 - t22665 - t22669 - t22674 + t32995 + t32996 + t32997 - t19709 + 0.4051561992e0 * t29028 + 0.8103123984e0 * t29030;
    (t33785, t33790)
}
