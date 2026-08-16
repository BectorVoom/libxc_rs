//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 609/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk609(t169: f64, t1646: f64, t113: f64, t2633: f64, t171: f64, t2629: f64, t1650: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t170 = t169 <= zeta_threshold;
    let t6272 = t1646 * t1646;
    let t6276 = 2.0_f64 * t113 + 2.0_f64 * t2633;
    let t6280 = piecewise3(t170, 0.0_f64, 4.0_f64 / 9.0_f64 * t2629 * t6272 + 4.0_f64 / 3.0_f64 * t171 * t6276);
    let t6281 = t1650 * t1650;
    (t6272, t6276, t6280, t6281)
}
