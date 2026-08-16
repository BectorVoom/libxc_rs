//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 876/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk876<F: Float>(t2137: F, t5389: F, t467: F, t2138: F, t5326: F, t800: F, t8171: F, t26865: F, t4890: F, t3767: F, t3782: F, t1227: F, t1238: F, t1266: F, t26867: F, t26870: F, t26877: F, t5335: F, t5343: F, t5348: F, t5354: F, t5369: F, t5397: F, t5402: F, t7607: F, t7624: F) -> F {
    let t29082 = t2137 * t5389;
    let t29083 = t467 * t29082;
    let t29086 = t5326 * t2138;
    let t29089 = t8171 * t800;
    let t29096 = t26865 * t4890;
    let t29097 = t3767 * t29096;
    let t29100 = t3782 * t29096;
    let t29107 = -F::cast_from(0.28582678745379824648e-3_f64) * t7624 * t5397 + F::cast_from(0.15244095330869239812e-2_f64) * t29083 * t1266 - F::cast_from(0.42874018118069736972e-3_f64) * t29086 * t1238 + t29089 * t1227 / F::cast_from(108.0_f64) - t7607 * t5369 / F::cast_from(288.0_f64) - t26877 - F::cast_from(0.28582678745379824648e-3_f64) * t26867 * t5402 + F::cast_from(0.85748036236139473944e-3_f64) * t29097 * t5343 - F::cast_from(0.42874018118069736972e-3_f64) * t29100 * t5335 - F::cast_from(0.42874018118069736972e-3_f64) * t26870 * t5348 - F::cast_from(0.42874018118069736972e-3_f64) * t26870 * t5354;
    t29107
}
