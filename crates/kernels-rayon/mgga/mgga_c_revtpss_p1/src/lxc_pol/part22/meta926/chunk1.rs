//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3150/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3150(t1261: f64, t17236: f64, t3172: f64, t17540: f64, t3711: f64, t12956: f64, t17209: f64, t17198: f64, t12773: f64, t17605: f64, t17557: f64, t17535: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t56793 = t1261 * t3172 * t17236;
    let t56796 = t3711 * t3172 * t17540;
    let t56798 = t12956 * t17209;
    let t56812 = t1261 * t3172 * t17198;
    let t56835 = t17605 * t12773;
    let t56838 = t1261 * t3172 * t17557;
    let t56853 = t3711 * t3172 * t17535;
    (t56793, t56796, t56798, t56812, t56835, t56838, t56853)
}
