//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 932/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk932<F: Float>(t34547: F, t34549: F, t34578: F, t34590: F, t34650: F, t34698: F, t34722: F, t34724: F, t34738: F, t34751: F, t34804: F, t34844: F, t34879: F, t34897: F, t35022: F, t35043: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t37147 = 0.34299214494455789578e-2 * t34547;
    let t37148 = 0.16006300097412701803e-1 * t34549;
    let t37163 = 0.62896184579208304134e-2 * t34578;
    let t37166 = 0.17149607247227894789e-2 * t34590;
    let t37190 = 0.22921875e-1 * t34650;
    let t37211 = 0.42874018118069736972e-2 * t34698;
    let t37220 = 0.31448092289604152068e-2 * t34722;
    let t37221 = 0.18868855373762491241e-1 * t34724;
    let t37225 = 0.25724410870841842184e-2 * t34738;
    let t37233 = 0.34299214494455789578e-2 * t34751;
    let t37252 = 0.20965394859736101378e-2 * t34804;
    let t37271 = 0.34299214494455789578e-2 * t34844;
    let t37287 = 0.85748036236139473944e-3 * t34879;
    let t37293 = 0.13073958333333333333e0 * t34897;
    let t37345 = 0.57165357490759649296e-3 * t35022;
    let t37363 = 35.0 / 108.0 * t35043;
    (t37147, t37148, t37163, t37166, t37190, t37211, t37220, t37221, t37225, t37233, t37252, t37271, t37287, t37293, t37345, t37363)
}
