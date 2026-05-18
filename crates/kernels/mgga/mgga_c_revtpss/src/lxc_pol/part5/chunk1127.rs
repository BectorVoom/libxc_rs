//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1127/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1127<F: Float>(t17350: F, t3782: F, t1263: F, t1794: F, t372: F, t11262: F, t1796: F, t1247: F, t12915: F, t247: F, t5230: F, t5384: F) -> (F, F, F, F) {
    let t17351 = t3782 * t17350;
    let t17352 = t1263 * t1794;
    let t17353 = t372 * t17352;
    let t17361 = t11262 * t1796;
    let t17362 = t1247 * t17361;
    let t17373 = t247 * t12915 * t5230;
    let t17375 = F::new(0.57165357490759649296e-3) * t5384 * t17373;
    (t17351, t17353, t17362, t17375)
}
