//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1091/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1091<F: Float>(t38267: F, t38269: F, t38281: F, t38297: F, t38311: F, t38322: F, t38336: F, t38341: F, t38346: F, t38349: F, t38362: F, t3261: F, t5086: F, t97: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t39114 = F::cast_from(0.18292589874945016987e-2_f64) * t38267;
    let t39115 = F::cast_from(0.487802396665200453e-2_f64) * t38269;
    let t39116 = F::cast_from(0.13010691197123848592e-3_f64) * t38281;
    let t39117 = F::cast_from(0.18292589874945016987e-2_f64) * t38297;
    let t39121 = F::cast_from(0.13010691197123848592e-3_f64) * t38311;
    let t39122 = F::cast_from(0.26021382394247697185e-3_f64) * t38322;
    let t39127 = F::cast_from(0.2439011983326002265e-2_f64) * t38336;
    let t39129 = F::cast_from(0.2439011983326002265e-2_f64) * t38341;
    let t39130 = F::cast_from(0.18292589874945016987e-2_f64) * t38346;
    let t39131 = F::cast_from(0.30487649791575028312e-3_f64) * t38349;
    let t39134 = F::cast_from(0.91462949374725084936e-3_f64) * t38362;
    let t39190 = t97 * t3261 * t5086;
    (t39114, t39115, t39116, t39117, t39121, t39122, t39127, t39129, t39130, t39131, t39134, t39190)
}
