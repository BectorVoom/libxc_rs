//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 858/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk858<F: Float>(t2137: F, t5389: F, t467: F, t2138: F, t5326: F, t800: F, t8171: F, t26865: F, t4890: F, t3767: F, t3782: F, t1227: F, t1238: F, t1266: F, t26867: F, t26870: F, t26877: F, t5335: F, t5343: F, t5348: F, t5354: F, t5369: F, t5397: F, t5402: F, t7607: F, t7624: F) -> (F,) {
    let t29082 = t2137 * t5389;
    let t29083 = t467 * t29082;
    let t29086 = t5326 * t2138;
    let t29089 = t8171 * t800;
    let t29096 = t26865 * t4890;
    let t29097 = t3767 * t29096;
    let t29100 = t3782 * t29096;
    let t29107 = -0.28582678745379824648e-3 * t7624 * t5397 + 0.15244095330869239812e-2 * t29083 * t1266 - 0.42874018118069736972e-3 * t29086 * t1238 + t29089 * t1227 / 108.0 - t7607 * t5369 / 288.0 - t26877 - 0.28582678745379824648e-3 * t26867 * t5402 + 0.85748036236139473944e-3 * t29097 * t5343 - 0.42874018118069736972e-3 * t29100 * t5335 - 0.42874018118069736972e-3 * t26870 * t5348 - 0.42874018118069736972e-3 * t26870 * t5354;
    (t29107,)
}
