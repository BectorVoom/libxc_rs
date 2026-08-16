//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 719/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk719(t533: f64, t8807: f64, t3701: f64, t113: f64, t1983: f64, t2036: f64, t2040: f64, t2075: f64, t2096: f64, t510: f64, t574: f64, t652: f64, t7042: f64, t8329: f64, t8607: f64, t8711: f64, t8718: f64, t8721: f64, t8774: f64, t8780: f64, t8805: f64) -> (f64, f64, f64) {
    let t8808 = t533 * t8807;
    let t8809 = t8808 * t3701;
    let t8811 = -t113 * t8774 + t1983 * t8805 - t1983 * t8809 - 2.0_f64 * t2036 * t2075 - 4.0_f64 * t2040 * t7042 + 2.0_f64 * t2096 * t8607 - t510 * t8711 - 2.0_f64 * t510 * t8718 + t574 * t8780 - 4.0_f64 * t652 * t8721 - t8329;
    (t8808, t8809, t8811)
}
