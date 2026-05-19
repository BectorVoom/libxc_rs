//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1090/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1090<F: Float>(t37480: F, t37523: F, t37527: F, t37531: F, t37541: F, t37560: F, t37568: F, t38225: F, t38228: F, t38233: F, t38244: F, t38264: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t39083 = F::cast_from(0.26021382394247697185e-3_f64) * t37480;
    let t39091 = F::cast_from(0.12649025447177706166e-6_f64) * t37523;
    let t39092 = F::cast_from(0.89430439388620083049e-2_f64) * t37527;
    let t39093 = F::cast_from(0.3286404220903135089e-2_f64) * t37531;
    let t39094 = F::cast_from(0.487802396665200453e-2_f64) * t37541;
    let t39097 = F::cast_from(0.2439011983326002265e-2_f64) * t37560;
    let t39099 = F::cast_from(0.30487649791575028312e-3_f64) * t37568;
    let t39106 = F::cast_from(0.18292589874945016987e-2_f64) * t38225;
    let t39107 = F::cast_from(0.1299607316140891005e-4_f64) * t38228;
    let t39108 = F::cast_from(0.11709622077411463733e-2_f64) * t38233;
    let t39109 = F::cast_from(0.205201155180140685e-5_f64) * t38244;
    let t39113 = F::cast_from(0.30487649791575028312e-3_f64) * t38264;
    (t39083, t39091, t39092, t39093, t39094, t39097, t39099, t39106, t39107, t39108, t39109, t39113)
}
