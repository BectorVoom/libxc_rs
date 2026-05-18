//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1228/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1228<F: Float>(t1851: F, t4621: F, t15239: F, t11081: F, t6762: F, t3514: F, t1262: F, t1662: F, t11072: F, t330: F, t6774: F, t829: F) -> (F, F, F, F) {
    let t20563 = t4621 * t1851;
    let t20564 = t15239 * t20563;
    let t20569 = t11081 * t6762;
    let t20570 = t3514 * t20569;
    let t20572 = t1851 * t1262;
    let t20573 = t1662 * t20572;
    let t20574 = t11072 * t20573;
    let t20578 = t6774 * t330;
    let t20579 = t20578 * t829;
    (t20564, t20570, t20574, t20579)
}
