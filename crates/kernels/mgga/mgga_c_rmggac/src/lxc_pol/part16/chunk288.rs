//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 288/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk288<F: Float>(t1652: F, t338: F, t118: F, t1594: F, t1596: F, t1600: F, t1603: F, t1605: F, t1607: F, t1609: F, t1616: F, t1618: F, t1620: F, t1622: F) -> (F, F) {
    let t1653 = t338 * t1652;
    let t1654 = t118 * t1653;
    let t1656 = -F::new(0.11974241701863808564e0) * t1594 + F::new(0.17961362552795712846e0) * t1596 + F::new(0.59871208509319042821e-1) * t1600 - F::new(0.59871208509319042821e-1) * t1603 + F::new(0.17961362552795712846e0) * t1605 - F::new(0.23948483403727617128e0) * t1607 - F::new(0.59871208509319042821e-1) * t1609 + F::new(0.59871208509319042821e-1) * t1616 + F::new(0.59871208509319042821e-1) * t1618 - F::new(0.59871208509319042821e-1) * t1620 - F::new(0.39914139006212695214e-1) * t1622 + F::new(0.19957069503106347607e-1) * t1654;
    (t1654, t1656)
}
