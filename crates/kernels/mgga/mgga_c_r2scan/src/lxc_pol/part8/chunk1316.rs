//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1316/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1316<F: Float>(t166: F, t32195: F, t607: F, t9904: F, t28020: F, t19013: F, t19032: F, t23753: F, t23759: F, t23761: F, t23763: F, t31378: F, t32146: F, t32149: F, t881: F, t23764: F) -> (F, F, F, F, F) {
    let t32196 = t32195 * t166;
    let t32199 = t9904 * t607;
    let t32202 = 3.0 * t28020;
    let t32204 = -t32146 - t19013 + t23753 + t23759 + t23761 - t32149 + t23763 - 0.2363e1 * t881 * t32196 - 0.2363e1 * t881 * t32199 - t32202 - t19032 - 0.7089e1 * t31378;
    let t32207 = 0.10526802520742363173e2 * t23764;
    (t32196, t32199, t32202, t32204, t32207)
}
