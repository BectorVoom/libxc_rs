//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1756/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1756<F: Float>(t3704: F, t5293: F, t1802: F, t3147: F, t3597: F, t3594: F, t1244: F, t3172: F, t5286: F, t1247: F, t3707: F, t5292: F) -> (F, F, F, F, F, F, F, F) {
    let t17509 = F::cast_from(0.15244095330869239812e-2_f64) * t5293 * t3704;
    let t17523 = t1802 * t3147;
    let t17524 = t3597 * t17523;
    let t17525 = t3594 * t17524;
    let t17528 = t1244 * t17523;
    let t17529 = t3594 * t17528;
    let t17544 = t3172 * t5286;
    let t17546 = F::cast_from(0.28582678745379824648e-3_f64) * t1247 * t17544;
    let t17547 = t3707 * t5292;
    (t17509, t17524, t17525, t17528, t17529, t17544, t17546, t17547)
}
