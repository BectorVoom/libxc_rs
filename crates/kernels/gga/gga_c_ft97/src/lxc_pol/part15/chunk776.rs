//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 776/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk776<F: Float>(t200: F, t21373: F, t4951: F, t5001: F, t4978: F, t6: F, t17850: F, t223: F, t13411: F, t1097: F, t13414: F, t13582: F, t13586: F, t17825: F, t17833: F, t17847: F, t17851: F, t18090: F, t18133: F, t21235: F, t21325: F, t21330: F, t21331: F, t21333: F, t21338: F, t238: F, t2387: F, t4949: F, t4950: F, t4952: F, t678: F, t680: F) -> (F, F, F, F, F, F) {
    let t21374 = t21373 * t200;
    let t21382 = t4951 * t5001;
    let t21386 = t4978 * t6;
    let t21392 = t17850 * t223;
    let t21393 = t13411 * t21392;
    let t21396 = F::cast_from(0.34882351419656688e-1_f64) * t2387 * t21325 - F::cast_from(0.49022040019937983366e-5_f64) * t21330 * t21331 * t21333 + F::cast_from(0.13774561697978600408e-4_f64) * t17833 * t21338 + F::cast_from(0.20279640676073749279e-3_f64) * t17847 * t18133 + F::cast_from(0.40559281352147498558e-3_f64) * t17851 * t13582 - F::cast_from(0.20279640676073749279e-3_f64) * t17851 * t13586 - F::cast_from(0.11627450473218896e-1_f64) * t678 * t680 * t21374 - F::cast_from(0.69764702839313376e-1_f64) * t18090 * t1097 + F::cast_from(0.41352194951222972388e-4_f64) * t17825 * t21338 + F::cast_from(0.13094861734553941256e-2_f64) * t4949 * t4950 * t21382 - F::cast_from(0.20676097475611486194e-3_f64) * t4949 * t21386 * t4952 + F::cast_from(0.27529390119979671431e0_f64) * t238 * t21235 + F::cast_from(0.48082059875423759229e-5_f64) * t21393 * t13414;
    (t21374, t21382, t21386, t21392, t21393, t21396)
}
