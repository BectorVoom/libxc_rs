//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 320/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk320<F: Float>(t1123: F, t301: F, t761: F, t758: F, t1066: F, t179: F, t780: F, t1120: F, t276: F, t299: F, t741: F, t757: F, t777: F) -> (F, F, F, F, F) {
    let t1124 = t301 * t1123;
    let t1125 = t1124 * t761;
    let t1126 = t758 * t1125;
    let t1130 = t179 * t780 * t1066;
    let t1133 = t741 - t276 * t1120 / F::new(96.0) + F::cast_from(0.21437009059034868486e-3_f64) * t757 * t1126 + t777 - F::cast_from(0.42874018118069736972e-3_f64) * t299 * t1130;
    (t1124, t1125, t1126, t1130, t1133)
}
