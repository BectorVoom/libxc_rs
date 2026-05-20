//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3275/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3275<F: Float>(t14662: F, t2723: F, t14671: F, t14686: F, t14931: F, t18632: F, t14494: F, t14791: F, t1559: F, t18426: F, t18637: F, t2477: F, t2745: F, t2754: F, t4362: F, t4364: F, t4365: F, t51095: F, t51098: F, t51100: F, t51102: F, t51104: F, t51106: F, t61234: F, t62080: F, t828: F, t851: F) -> (F, F) {
    let t62209 = t2723 * t14662;
    let t62216 = t14931 * t14686 * t14671 * t18632;
    let t62231 = F::cast_from(0.85748036236139473944e-2_f64) * t851 * t2477 * t828 * t61234 + F::cast_from(0.34299214494455789578e-2_f64) * t2745 * t14791 * t14494 * t18637 - F::new(35.0) / F::new(54.0) * t51095 + F::cast_from(0.85748036236139473944e-3_f64) * t4362 * t4364 * t4365 * t62209 + F::cast_from(0.10164000561857065645e-3_f64) * t62216 - F::cast_from(0.21437009059034868486e-3_f64) * t2745 * t4364 * t18426 * t2754 + F::cast_from(0.34299214494455789578e-2_f64) * t2745 * t14791 * t1559 * t62080 - F::cast_from(0.22675591804667994222e-1_f64) * t51098 - F::cast_from(0.25692334753583138158e-2_f64) * t51100 + F::cast_from(0.1219527626469539185e-2_f64) * t51102 + F::cast_from(0.7558530601555998074e-1_f64) * t51104 + F::new(7.0) / F::new(6.0) * t51106;
    (t62209, t62231)
}
