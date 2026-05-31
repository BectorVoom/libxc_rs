//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2668/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2668<F: Float>(t13946: F, t9962: F, t1413: F, t46835: F, t48694: F, t13775: F, t9793: F, t9794: F, t5690: F, t9741: F, t14016: F, t46691: F) -> (F, F, F, F, F) {
    let t49118 = t9962 * t13946;
    let t49121 = t46835 * t1413 * t48694;
    let t49122 = F::cast_from(0.30492001685571196935e-4_f64) * t49121;
    let t49124 = t9793 * t9794 * t13775;
    let t49125 = F::cast_from(0.13553694749236397037e-4_f64) * t49124;
    let t49126 = t9741 * t5690;
    let t49127 = F::cast_from(35.0_f64) / F::cast_from(72.0_f64) * t49126;
    let t49128 = t46691 * t14016;
    (t49118, t49122, t49125, t49127, t49128)
}
