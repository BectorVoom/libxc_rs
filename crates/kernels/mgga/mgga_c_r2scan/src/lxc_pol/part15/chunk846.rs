//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 846/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk846<F: Float>(t2148: F, t7629: F, t7628: F, t1398: F, t5: F, t966: F, t2804: F, t378: F, t5202: F, t5205: F, t5209: F, t5212: F, t5213: F, t5218: F, t5220: F, t5225: F, t5230: F, t5233: F, t5235: F) -> (F, F, F, F) {
    let t7630 = t2148 * t7629;
    let t7632 = F::cast_from(0.23287303101564395622e-1_f64) * t7628 * t7630;
    let t7637 = t5 * t1398 * t966;
    let t7641 = F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t5 * t378 * t2804;
    let t7645 = -t5202 - t5205 - t5209 + t5212 + F::cast_from(0.53360572013155555553e-2_f64) * t5213 - t5218 - F::cast_from(0.67745118933333333332e-2_f64) * t5220 - t5225 + t5230 - t5233 - F::cast_from(0.54217906501508699211e-2_f64) * t5235;
    (t7632, t7637, t7641, t7645)
}
