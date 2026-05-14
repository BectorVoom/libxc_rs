//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 974/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk974<F: Float>(t37531: F, t37541: F, t37560: F, t37568: F, t38225: F, t38228: F, t38233: F, t38244: F, t38264: F, t38267: F, t38269: F, t38281: F, t38297: F, t38311: F, t38322: F, t38336: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t39093 = 0.3286404220903135089e-2 * t37531;
    let t39094 = 0.487802396665200453e-2 * t37541;
    let t39097 = 0.2439011983326002265e-2 * t37560;
    let t39099 = 0.30487649791575028312e-3 * t37568;
    let t39106 = 0.18292589874945016987e-2 * t38225;
    let t39107 = 0.1299607316140891005e-4 * t38228;
    let t39108 = 0.11709622077411463733e-2 * t38233;
    let t39109 = 0.205201155180140685e-5 * t38244;
    let t39113 = 0.30487649791575028312e-3 * t38264;
    let t39114 = 0.18292589874945016987e-2 * t38267;
    let t39115 = 0.487802396665200453e-2 * t38269;
    let t39116 = 0.13010691197123848592e-3 * t38281;
    let t39117 = 0.18292589874945016987e-2 * t38297;
    let t39121 = 0.13010691197123848592e-3 * t38311;
    let t39122 = 0.26021382394247697185e-3 * t38322;
    let t39127 = 0.2439011983326002265e-2 * t38336;
    (t39093, t39094, t39097, t39099, t39106, t39107, t39108, t39109, t39113, t39114, t39115, t39116, t39117, t39121, t39122, t39127)
}
