//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 927/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk927<F: Float>(t322: F, t10533: F, t11148: F, t11162: F, t12002: F, t12009: F, t12622: F, t12624: F, t12627: F, t12629: F, t12656: F, t12681: F, t12683: F, t12692: F, t330: F, t3413: F, t3420: F, t352: F, t3675: F, t855: F) -> (F,) {
    let t323 = t322 <= 0.0;
    let t331 = t322 <= 0.25e1;
    let t12703 = piecewise5(t323, t12622 * t330 + 2.0 * t12624 * t330 + t12627 * t330 + t12629 * t330, t331, t12656 + t12681, -0.63e1 * t3420 * t12683 - 0.42e1 * t12002 * t3675 - 0.945e1 * t11148 * t12683 - 0.21e1 * t3413 * t10533 - 0.105e1 * t855 * t12692 * t352 - 0.315e1 * t12009 * t3675 - 0.1575e1 * t3420 * t10533 - 0.23625e1 * t11162 * t12683);
    (t12703,)
}
