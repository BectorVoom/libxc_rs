//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 584/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk584(t169: f64, t174: f64, t171: f64, t2629: f64, t6272: f64, t6276: f64, t1650: f64, t176: f64, t2641: f64, t44: f64, t234: f64, t1709: f64, t2811: f64, t313: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t170 = t169 <= zeta_threshold;
    let t175 = t174 <= zeta_threshold;
    let t6280 = piecewise3(t170, 0.0_f64, 4.0_f64 / 9.0_f64 * t2629 * t6272 + 4.0_f64 / 3.0_f64 * t171 * t6276);
    let t6281 = t1650 * t1650;
    let t6284 = -t6276;
    let t6288 = piecewise3(t175, 0.0_f64, 4.0_f64 / 9.0_f64 * t2641 * t6281 + 4.0_f64 / 3.0_f64 * t176 * t6284);
    let t6290 = (t6280 + t6288) * t44;
    let t6293 = piecewise3(t170, 0.0_f64, t6276);
    let t6294 = t234 * t6293;
    let t6301 = t1709 * t1709;
    let t6302 = t6301 * t2811;
    let t6307 = t313 * t6272;
    (t6281, t6284, t6290, t6293, t6294, t6301, t6302, t6307)
}
