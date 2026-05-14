//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1189/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1189<F: Float>(t17443: F, t5905: F, t16673: F, t4293: F, t4292: F, t1466: F, t5997: F, t1535: F, t1489: F, t5875: F, t5904: F, t16653: F, t15898: F, t4261: F, t4260: F, t11825: F, t4291: F, sigma2: F) -> (F, F, F, F, F, F, F, F) {
    let t17444 = t17443 * t5905;
    let t17446 = t4293 * t16673;
    let t17447 = t4292 * t17446;
    let t17449 = t5997 * t1466;
    let t17450 = t17449 * sigma2;
    let t17451 = t17450 * t1535;
    let t17453 = t5875 * t1489;
    let t17454 = t5904 * t17453;
    let t17455 = t4292 * t17454;
    let t17457 = t4293 * t16653;
    let t17458 = t4292 * t17457;
    let t17460 = t4261 * t15898;
    let t17461 = t4260 * t17460;
    let t17463 = t11825 * t4291;
    (t17444, t17447, t17451, t17453, t17455, t17458, t17461, t17463)
}
