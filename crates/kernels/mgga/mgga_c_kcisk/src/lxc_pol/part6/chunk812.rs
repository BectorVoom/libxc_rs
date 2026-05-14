//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 812/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk812<F: Float>(t1746: F, t8763: F, t7156: F, t10913: F, t29102: F, t4957: F, t1248: F, t28377: F, t4893: F, t1720: F, t28385: F, t2408: F, t8701: F, t11036: F, t11056: F, t11040: F, t17382: F, t23460: F, t23472: F, t23481: F, t29082: F, t29085: F, t29088: F, t29091: F, t29094: F, t29097: F) -> (F, F, F, F, F, F, F) {
    let t29107 = t1746 * t8763;
    let t29108 = t7156 * t29107;
    let t29111 = t10913 * t29102;
    let t29112 = t29111 * t4957;
    let t29116 = t1248 * t4893 * t28377;
    let t29121 = t1248 * t1720 * t28385;
    let t29123 = t8701 * t2408;
    let t29124 = t11036 * t29123;
    let t29126 = t11056 * t29123;
    let t29138 = -t11040 - 4.0 / 9.0 * t17382 + 2.0 / 9.0 * t23460 - 2.0 / 3.0 * t23472 + t23481 / 3.0 - 10.0 / 27.0 * t29082 + 4.0 / 3.0 * t29085 - 2.0 / 3.0 * t29088 - 2.0 * t29091 + 2.0 * t29094 - t29097 / 3.0;
    (t29108, t29112, t29116, t29121, t29124, t29126, t29138)
}
