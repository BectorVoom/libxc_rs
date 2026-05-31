//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 902/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk902<F: Float>(t449: F, t6260: F, t446: F, t4529: F, t113: F, t774: F, t2150: F, t62: F, t822: F, t251: F, t4863: F, t2532: F) -> (F, F, F, F, F, F, F) {
    let t6261 = t449 * t6260;
    let t6262 = t446 * t6261;
    let t6292 = F::cast_from(2.0_f64) * t4529;
    let t7617 = t113 * t774;
    let t7624 = t2150 * t774;
    let t7627 = t62 * t822;
    let t8291 = t251 * t4863;
    let t8521 = F::cast_from(3.0_f64) * t2532;
    (t6262, t6292, t7617, t7624, t7627, t8291, t8521)
}
