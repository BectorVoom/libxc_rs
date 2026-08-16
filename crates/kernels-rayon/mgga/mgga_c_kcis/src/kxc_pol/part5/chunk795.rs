//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 795/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk795(t169: f64, t174: f64, t176: f64, t2641: f64, t6281: f64, t6284: f64, t44: f64, t6280: f64, t230: f64, t6276: f64, t234: f64, t441: f64, t233: f64, t1658: f64, t1876: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t170 = t169 <= zeta_threshold;
    let t175 = t174 <= zeta_threshold;
    let t6288 = piecewise3(t175, 0.0_f64, 4.0_f64 / 9.0_f64 * t2641 * t6281 + 4.0_f64 / 3.0_f64 * t176 * t6284);
    let t6290 = (t6280 + t6288) * t44;
    let t6291 = t6290 * t230;
    let t6293 = piecewise3(t170, 0.0_f64, t6276);
    let t6294 = t234 * t6293;
    let t6295 = t6294 * t441;
    let t6296 = t233 * t6295;
    let t6297 = t6296 / 16.0_f64;
    let t6298 = t1658 * t1876;
    (t6290, t6291, t6294, t6295, t6297, t6298)
}
