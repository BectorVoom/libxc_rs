//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1213/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1213<F: Float>(t32094: F, t792: F, t37327: F, t4176: F, t11502: F, t37346: F, t1561: F, t3274: F, t97: F, t32212: F, t14160: F, t1234: F, t2867: F) -> (F, F, F, F) {
    let t40566 = t32094 * t792;
    let t40569 = F::new(15.0) / F::new(8.0) * t37327 * t4176 * t40566;
    let t40571 = F::new(3.0) / F::new(4.0) * t37346 * t11502;
    let t40574 = t97 * t3274 * t1561;
    let t40575 = t32212 * t792;
    let t40578 = F::new(5.0) / F::new(4.0) * t40574 * t14160 * t40575;
    let t40579 = t2867 * t1234;
    (t40569, t40571, t40578, t40579)
}
