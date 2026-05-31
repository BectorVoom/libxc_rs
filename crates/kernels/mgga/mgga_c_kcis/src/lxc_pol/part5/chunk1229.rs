//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1229/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1229<F: Float>(t11072: F, t20579: F, t330: F, t6837: F, t829: F, t3515: F, t25: F, t6775: F, t1251: F, t1262: F, t6330: F, t11063: F, t11086: F, t11093: F, t11100: F, t20564: F, t20570: F, t20574: F, t3490: F, t3514: F, t6763: F, t6776: F) -> F {
    let t20580 = t11072 * t20579;
    let t20583 = t6837 * t330;
    let t20584 = t20583 * t829;
    let t20585 = t3515 * t20584;
    let t20590 = t25 * t6775;
    let t20591 = t1251 * t20590;
    let t20593 = t6330 * t1262;
    let t20594 = t3515 * t20593;
    let t20598 = -t3514 * t20564 / F::cast_from(144.0_f64) + t11086 * t6763 / F::cast_from(108.0_f64) - t20570 / F::cast_from(864.0_f64) + t3514 * t20574 / F::cast_from(144.0_f64) - t11063 / F::cast_from(2592.0_f64) + t3514 * t20580 / F::cast_from(288.0_f64) - t3514 * t20585 / F::cast_from(576.0_f64) - t3490 * t6776 / F::cast_from(36.0_f64) + t20591 / F::cast_from(288.0_f64) + t3514 * t20594 / F::cast_from(288.0_f64) + t11093 + t11100 / F::cast_from(324.0_f64);
    t20598
}
