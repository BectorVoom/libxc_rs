//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2703/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2703<F: Float>(t13652: F, t177: F, t6800: F, t762: F, t13666: F, t13668: F, t9858: F, t9861: F, t13887: F, t13664: F, t13682: F, t13683: F, t9524: F, t9542: F, t9588: F, t9854: F, t9865: F, t9868: F) -> (F, F, F, F, F, F, F, F, F) {
    let t22211 = F::cast_from(0.34631718211362927517e2_f64) * t13652;
    let t22212 = t6800 * t177;
    let t22213 = t22212 * t762;
    let t22214 = F::cast_from(0.5848223622634646207e0_f64) * t22213;
    let t22215 = F::cast_from(0.21687162600603479684e-1_f64) * t13666;
    let t22216 = F::cast_from(24.0_f64) * t13668;
    let t22217 = F::cast_from(0.17315859105681463759e2_f64) * t9858;
    let t22218 = F::cast_from(0.10843581300301739842e-1_f64) * t9861;
    let t22219 = F::cast_from(0.48830526149350786811e-3_f64) * t13887;
    let t22220 = -t22211 - t9588 - t9524 - t13664 - t22214 + t22215 - t22216 + t9542 + t13682 + t9854 + t13683 - t22217 + t22218 + t9865 + t9868 + t22219;
    (t22211, t22212, t22214, t22215, t22216, t22217, t22218, t22219, t22220)
}
