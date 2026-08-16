//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 776/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk776(t200: f64, t21373: f64, t4951: f64, t5001: f64, t4978: f64, t6: f64, t17850: f64, t223: f64, t13411: f64, t1097: f64, t13414: f64, t13582: f64, t13586: f64, t17825: f64, t17833: f64, t17847: f64, t17851: f64, t18090: f64, t18133: f64, t21235: f64, t21325: f64, t21330: f64, t21331: f64, t21333: f64, t21338: f64, t238: f64, t2387: f64, t4949: f64, t4950: f64, t4952: f64, t678: f64, t680: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t21374 = t21373 * t200;
    let t21382 = t4951 * t5001;
    let t21386 = t4978 * t6;
    let t21392 = t17850 * t223;
    let t21393 = t13411 * t21392;
    let t21396 = 0.34882351419656688e-1_f64 * t2387 * t21325 - 0.49022040019937983366e-5_f64 * t21330 * t21331 * t21333 + 0.13774561697978600408e-4_f64 * t17833 * t21338 + 0.20279640676073749279e-3_f64 * t17847 * t18133 + 0.40559281352147498558e-3_f64 * t17851 * t13582 - 0.20279640676073749279e-3_f64 * t17851 * t13586 - 0.11627450473218896e-1_f64 * t678 * t680 * t21374 - 0.69764702839313376e-1_f64 * t18090 * t1097 + 0.41352194951222972388e-4_f64 * t17825 * t21338 + 0.13094861734553941256e-2_f64 * t4949 * t4950 * t21382 - 0.20676097475611486194e-3_f64 * t4949 * t21386 * t4952 + 0.27529390119979671431e0_f64 * t238 * t21235 + 0.48082059875423759229e-5_f64 * t21393 * t13414;
    (t21374, t21382, t21386, t21392, t21393, t21396)
}
