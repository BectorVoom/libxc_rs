//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1168/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1168<F: Float>(t1409: F, t21585: F, t5526: F, t5792: F, t17057: F, t1961: F, t7119: F, t833: F, t6284: F, t1419: F, t7123: F, t11939: F, t7122: F, t5804: F, t7142: F, t4035: F, t7141: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t21586 = t1409 * t21585;
    let t21594 = t5792 * t5526;
    let t21597 = t17057 * t1961;
    let t21600 = t7119 * t833;
    let t21603 = t1409 * t6284;
    let t21604 = t21603 * t1419;
    let t21607 = t7123 * t833;
    let t21610 = t11939 * t7122;
    let t21611 = t21610 * t1419;
    let t21614 = t5804 * t5526;
    let t21617 = t7142 * t833;
    let t21620 = t4035 * t7141;
    (t21586, t21594, t21597, t21600, t21604, t21607, t21611, t21614, t21617, t21620)
}
