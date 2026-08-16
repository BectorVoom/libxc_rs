//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3246/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3246<F: Float>(t6016: F, t853: F, t2661: F, t2662: F, t2749: F, t18392: F, t2477: F, t40374: F, t40393: F, t40395: F, t40399: F, t40409: F, t40411: F, t50353: F, t50370: F, t50372: F, t50374: F, t775: F, t828: F, t851: F) -> F {
    let t61579 = t853 * t6016;
    let t61582 = t2661 * t2662 * t61579 * t2749;
    let t61599 = -F::cast_from(0.57165357490759649296e-4_f64) * t61582 + F::cast_from(0.85748036236139473944e-2_f64) * t851 * t2477 * t828 * t18392 * t775 + F::cast_from(0.80031500487063509015e-2_f64) * t50353 + F::cast_from(0.13552000749142754193e-3_f64) * t40374 - F::cast_from(0.56688979511669985553e-2_f64) * t40393 - F::cast_from(0.56688979511669985553e-2_f64) * t40395 + F::cast_from(0.11337795902333997111e-1_f64) * t40399 - F::cast_from(0.40164115440237189888e-6_f64) * t40409 + F::cast_from(0.60976381323476959248e-3_f64) * t40411 + F::cast_from(0.30234122406223992295e0_f64) * t50370 + F::cast_from(0.14450132032386466905e-2_f64) * t50372 - F::cast_from(0.30488190661738479624e-3_f64) * t50374;
    t61599
}
