//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3096/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3096(t3588: f64, t5341: f64, t12904: f64, t5293: f64, t12959: f64, t17569: f64, t11262: f64, t1261: f64, t5269: f64, t17236: f64, t3172: f64, t17540: f64, t3711: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t56766 = t5341 * t3588;
    let t56785 = t5293 * t12904;
    let t56786 = 0.7622047665434619906e-3_f64 * t56785;
    let t56787 = t17569 * t12959;
    let t56790 = t1261 * t11262 * t5269;
    let t56791 = 0.19055119163586549765e-3_f64 * t56790;
    let t56793 = t1261 * t3172 * t17236;
    let t56796 = t3711 * t3172 * t17540;
    (t56766, t56786, t56787, t56791, t56793, t56796)
}
