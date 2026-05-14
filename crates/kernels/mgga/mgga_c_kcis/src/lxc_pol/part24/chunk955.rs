//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 955/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk955<F: Float>(t27792: F, t7718: F, t1092: F, t26671: F, t8047: F, t1020: F, t2822: F, t8048: F, t14443: F, t8037: F, t7703: F, t291: F, t417: F, t1008: F, t13097: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t27793 = t7718 * t27792;
    let t27794 = t1092 * t27793;
    let t27796 = t26671 * t8047;
    let t27797 = t1020 * t27796;
    let t27799 = t2822 * t8048;
    let t27803 = t14443 * t8037;
    let t27804 = t7703 * t27803;
    let t27806 = t417 * t291;
    let t27807 = t13097 * t1008;
    let t27808 = t27806 * t27807;
    (t27793, t27794, t27796, t27797, t27799, t27803, t27804, t27806, t27807, t27808)
}
