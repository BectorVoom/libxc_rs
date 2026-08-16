//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1044/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1044(t34508: f64, t34510: f64, t34547: f64, t34549: f64, t34578: f64, t34590: f64, t34650: f64, t34698: f64, t34722: f64, t34724: f64, t34738: f64, t34751: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t37130 = 0.31448092289604152068e-2_f64 * t34508;
    let t37131 = 0.13208198761633743869e-1_f64 * t34510;
    let t37147 = 0.34299214494455789578e-2_f64 * t34547;
    let t37148 = 0.16006300097412701803e-1_f64 * t34549;
    let t37163 = 0.62896184579208304134e-2_f64 * t34578;
    let t37166 = 0.17149607247227894789e-2_f64 * t34590;
    let t37190 = 0.22921875e-1_f64 * t34650;
    let t37211 = 0.42874018118069736972e-2_f64 * t34698;
    let t37220 = 0.31448092289604152068e-2_f64 * t34722;
    let t37221 = 0.18868855373762491241e-1_f64 * t34724;
    let t37225 = 0.25724410870841842184e-2_f64 * t34738;
    let t37233 = 0.34299214494455789578e-2_f64 * t34751;
    (t37130, t37131, t37147, t37148, t37163, t37166, t37190, t37211, t37220, t37221, t37225, t37233)
}
