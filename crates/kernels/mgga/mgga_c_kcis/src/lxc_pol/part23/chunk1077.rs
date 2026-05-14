//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1077/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1077<F: Float>(t4298: F, t5748: F, t6029: F, t94805: F, t4303: F, t5752: F, t17370: F, t7948: F, t4307: F, t17396: F, t491: F, t27521: F, t17477: F, t27520: F, t27532: F, t28610: F) -> (F, F, F, F, F, F, F, F) {
    let t97671 = t5748 * t4298;
    let t97673 = t94805 * t6029;
    let t97675 = t5752 * t4303;
    let t97677 = t7948 * t17370;
    let t97679 = t5752 * t4307;
    let t97681 = t17396 * t491;
    let t97682 = t97681 * t27521;
    let t97684 = t27520 * t17477;
    let t97686 = t28610 * t27532;
    (t97671, t97673, t97675, t97677, t97679, t97682, t97684, t97686)
}
