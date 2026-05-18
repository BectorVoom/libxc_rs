//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 214/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk214<F: Float>(t218: F, t220: F, t675: F, t208: F, t655: F, t219: F, t657: F, t668: F, t670: F, t673: F) -> (F, F, F, F, F) {
    let t677 = t218 * t675 * t220;
    let t678 = F::new(0.82156666666666666667e-1) * t677;
    let t679 = t208 * t655;
    let t681 = t218 * t219 * t679;
    let t683 = F::new(0.1898925e1) * t668 - t670 + F::new(0.8969e0) * t657 + F::new(0.3071625e0) * t673 - t678 + F::new(0.24647e0) * t681;
    (t677, t678, t679, t681, t683)
}
