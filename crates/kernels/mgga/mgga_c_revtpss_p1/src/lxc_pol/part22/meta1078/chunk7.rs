//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3867/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3867<F: Float>(t22021: F, t9793: F, t9794: F, t13785: F, t46671: F, t46695: F, t46702: F, t46704: F, t46706: F, t46712: F, t48600: F, t48603: F, t48614: F, t5755: F, t73906: F, t73908: F) -> F {
    let t74341 = t9793 * t9794 * t22021;
    let t74347 = -F::cast_from(0.91476005056713590803e-4_f64) * t48600 + F::cast_from(0.10164000561857065645e-4_f64) * t48603 - F::cast_from(0.17149607247227894789e-1_f64) * t5755 * t73906 * t73908 * t13785 - F::cast_from(0.3659040202268543632e-3_f64) * t46671 - F::cast_from(0.22675591804667994222e-1_f64) * t48614 + F::cast_from(35.0_f64) / F::cast_from(72.0_f64) * t46695 - F::cast_from(0.45178982497454656791e-5_f64) * t74341 + F::cast_from(0.22589491248727328396e-6_f64) * t46702 + F::cast_from(0.15244095330869239812e-3_f64) * t46704 - F::cast_from(0.22675591804667994221e-1_f64) * t46706 - F::cast_from(0.27104001498285508386e-2_f64) * t46712;
    t74347
}
