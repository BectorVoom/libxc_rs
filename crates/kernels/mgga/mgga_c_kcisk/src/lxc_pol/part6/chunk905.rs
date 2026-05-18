//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 905/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk905<F: Float>(t2430: F, t8746: F, t1746: F, t4954: F, t8763: F, t7156: F, t10913: F, t4957: F, t1248: F, t28377: F, t4893: F, t1720: F, t28385: F) -> (F, F, F, F, F, F) {
    let t29102 = t8746 * t2430;
    let t29104 = t4954 * t29102 * t1746;
    let t29107 = t1746 * t8763;
    let t29108 = t7156 * t29107;
    let t29111 = t10913 * t29102;
    let t29112 = t29111 * t4957;
    let t29116 = t1248 * t4893 * t28377;
    let t29121 = t1248 * t1720 * t28385;
    (t29102, t29104, t29108, t29112, t29116, t29121)
}
