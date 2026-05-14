//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1457/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1457<F: Float>(t19013: F, t19032: F, t19037: F, t23753: F, t23759: F, t23761: F, t23763: F, t32149: F, t32202: F, t32207: F, t32208: F, t19041: F, t19048: F, t19057: F, t19061: F, t19069: F, t23781: F, t32209: F, t32210: F, t32215: F, t32217: F, t32218: F) -> (F, F) {
    let t35253 = t19013 - t23753 - t23759 - t23761 + t32149 - t23763 + t32202 + t19032 - t32207 + t19037 + t32208;
    let t35255 = -t19041 - t19048 + t32209 - t32210 - t19057 + t32215 + t19061 - t32217 + t32218 + t19069 - t23781;
    (t35253, t35255)
}
