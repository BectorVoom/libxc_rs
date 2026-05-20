//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2646/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2646<F: Float>(t46694: F, t5686: F, t14030: F, t9744: F, t13769: F, t808: F, t9736: F, t13952: F, t2689: F, t13784: F, t543: F, t46825: F, t9793: F) -> (F, F, F, F, F, F) {
    let t48685 = t46694 * t5686;
    let t48686 = F::new(35.0) / F::new(24.0) * t48685;
    let t48687 = t9744 * t14030;
    let t48690 = t9736 * t808 * t13769;
    let t48691 = F::cast_from(0.15246000842785598468e-3_f64) * t48690;
    let t48692 = t2689 * t13952;
    let t48694 = t13784 * t543;
    let t48696 = t9793 * t46825 * t48694;
    (t48686, t48687, t48691, t48692, t48694, t48696)
}
