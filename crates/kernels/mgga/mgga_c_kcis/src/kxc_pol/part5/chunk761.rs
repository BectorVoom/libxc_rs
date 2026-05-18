//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 761/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk761<F: Float>(t5835: F, t5866: F, t1360: F, t1404: F, t1455: F, t1924: F, t1979: F, t4018: F, t4019: F, t4021: F, t4023: F, t4059: F, t486: F, t510: F, t538: F, t5623: F, t5787: F, t5789: F, t5793: F, t5796: F, t5799: F, t5801: F, t5805: F, t5808: F) -> (F, F) {
    let t5867 = t5835 + t5866;
    let t5869 = t4018 + F::new(0.23426533963880895498e-2) * t4019 + F::new(0.46853067927761790996e-2) * t4021 + F::new(0.23426533963880895498e-2) * t5787 + F::new(0.46853067927761790996e-2) * t4023 * t5789 + F::new(0.46853067927761790996e-2) * t1404 * t5793 + F::new(0.46853067927761790996e-2) * t4059 * t5796 + F::new(0.46853067927761790996e-2) * t5799 + F::new(0.46853067927761790996e-2) * t1404 * t5801 + F::new(0.14055920378328537299e-1) * t510 * t5805 - F::new(0.46853067927761790996e-2) * t510 * t5808 - t5623 * t538 - t1924 * t1455 - t1360 * t1979 - t486 * t5867;
    (t5867, t5869)
}
