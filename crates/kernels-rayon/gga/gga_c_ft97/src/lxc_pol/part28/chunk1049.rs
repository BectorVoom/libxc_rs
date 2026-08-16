//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1049/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1049(t136531: f64, t136560: f64, t136920: f64, t136922: f64, t136926: f64, t136930: f64, t136967: f64, t136995: f64, t145372: f64, t145376: f64, t145379: f64, t145382: f64, t1624: f64, t25688: f64, t25774: f64, t25779: f64, t25780: f64, t25802: f64, t3057: f64, t32251: f64, t32252: f64, t32253: f64, t32295: f64, t32304: f64, t36364: f64, t37481: f64, t92354: f64, sigma0: f64) -> f64 {
    let t145409 = 0.26043295784446077722e-6_f64 * t136967 * t145376 + 0.25845121844514357744e-4_f64 * t32304 * t145379 + 0.28200083969358461042e-4_f64 * t136995 * t145382 + 0.13784064983740990797e-3_f64 * t32295 * t145372 - 0.17816121467177433866e-2_f64 * t136930 * t25780 + 0.79202200203119310706e-6_f64 * t136560 * t36364 * t25802 + 0.6595632919850939344e-7_f64 * t1624 * t92354 * t37481 * sigma0 * t36364 * t25774 + 0.79202200203119310706e-5_f64 * t136926 * t36364 * t25779 - 0.45497819271775541929e-4_f64 * t136920 * t136922 * t25688 + 0.22705522127871165896e-3_f64 * t136531 + 0.10338048737805743097e-3_f64 * t32251 * t32252 * t32253 * t3057;
    t145409
}
