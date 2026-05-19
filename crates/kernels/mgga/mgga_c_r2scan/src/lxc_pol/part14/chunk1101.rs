//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1101/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1101<F: Float>(t37414: F, t37442: F, t37447: F, t37451: F, t37458: F, t37460: F, t37463: F, t37472: F, t37480: F, t37523: F, t37527: F, t37531: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t39064 = F::cast_from(0.18292589874945016987e-2_f64) * t37414;
    let t39069 = F::cast_from(0.91462949374725084936e-3_f64) * t37442;
    let t39071 = F::cast_from(0.18292589874945016987e-2_f64) * t37447;
    let t39072 = F::cast_from(0.13911401682674235141e-1_f64) * t37451;
    let t39074 = F::cast_from(0.91462949374725084936e-3_f64) * t37458;
    let t39075 = F::cast_from(0.91462949374725084936e-3_f64) * t37460;
    let t39076 = F::cast_from(0.13010691197123848592e-3_f64) * t37463;
    let t39081 = F::cast_from(0.89430439388620083049e-2_f64) * t37472;
    let t39083 = F::cast_from(0.26021382394247697185e-3_f64) * t37480;
    let t39091 = F::cast_from(0.12649025447177706166e-6_f64) * t37523;
    let t39092 = F::cast_from(0.89430439388620083049e-2_f64) * t37527;
    let t39093 = F::cast_from(0.3286404220903135089e-2_f64) * t37531;
    (t39064, t39069, t39071, t39072, t39074, t39075, t39076, t39081, t39083, t39091, t39092, t39093)
}
