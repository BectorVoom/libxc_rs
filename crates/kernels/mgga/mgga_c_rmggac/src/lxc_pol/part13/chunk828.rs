//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 828/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk828<F: Float>(t2185: F, t8450: F, t2004: F, t2007: F, t1987: F, t1990: F, t209: F, t498: F, t16503: F, t321: F, t34962: F, t8440: F) -> (F, F, F, F, F, F) {
    let t38638 = t8450 * t2185;
    let t38639 = t38638 * t2004;
    let t38643 = t38638 * t2007;
    let t38645 = t38638 * t1987;
    let t38647 = t38638 * t1990;
    let t38649 = t209 * t498;
    let t38653 = t16503 * t34962 * t8440 * t38649 * t321;
    (t38639, t38643, t38645, t38647, t38649, t38653)
}
