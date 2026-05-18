//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1266/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1266<F: Float>(t12844: F, t27583: F, t28748: F, t27566: F, t28720: F, t27567: F, t99422: F, t18210: F, t28810: F, t7978: F, t99023: F, t98743: F) -> (F, F, F, F, F, F) {
    let t99556 = F::new(0.7722800925925925926e-4) * t27583 * t12844 * t28748;
    let t99565 = t28720 * t27566;
    let t99578 = F::new(0.10306077835648148148e-4) * t27567 * t99422;
    let t99591 = F::new(0.46336805555555555556e-3) * t7978 * t18210 * t28810;
    let t99593 = F::new(0.23168402777777777778e-3) * t7978 * t99023;
    let t99600 = F::new(0.15476481481481481481e-2) * t98743;
    (t99556, t99565, t99578, t99591, t99593, t99600)
}
