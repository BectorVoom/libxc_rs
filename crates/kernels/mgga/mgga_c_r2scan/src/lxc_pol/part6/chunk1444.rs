//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1444/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1444<F: Float>(t1543: F, t2562: F, t2148: F, t22868: F, t25569: F, t538: F, t6155: F, t20575: F, t7606: F, t26274: F, t19820: F, t2184: F, t2252: F, t22950: F, t22954: F, t22959: F, t22960: F, t22964: F, t22970: F, t2719: F, t495: F, t5109: F, t551: F, t552: F, t6465: F, t6583: F, t7964: F, t8117: F, t8215: F) -> (F,) {
    let t27182 = t2562 * t1543;
    let t27184 = t22868 * t2148 * t27182;
    let t27187 = t6155 * t538 * t25569;
    let t27198 = t20575 * t7606;
    let t27201 = t6155 * t538 * t26274;
    let t27211 = -0.20958572791407956061e0 * t27184 - 0.32927245914677557992e-1 * t27187 - 0.12459097221822660494e0 * t22950 - 0.43371823197556470519e-3 * t22954 + 0.26004665220162805689e0 * t2184 * t551 * t552 * t2719 * t2252 + 0.26004665220162805689e0 * t6465 * t8215 + 0.98781737744032673978e-1 * t27198 - 0.32927245914677557992e-1 * t27201 - 0.52009330440325611378e0 * t6583 * t5109 * t8117 * t495 - 0.7801399566048841707e0 * t19820 * t7964 - t22959 - 0.11557628986739024751e0 * t22960 - 0.58544643236296698111e-1 * t22964 - t22970;
    (t27211,)
}
