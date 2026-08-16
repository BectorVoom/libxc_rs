//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1169/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1169(t116: f64, t34873: f64, t13272: f64, t33362: f64, t124456: f64, t124463: f64, t124480: f64, t124483: f64, t125261: f64, t125269: f64, t125280: f64, t125283: f64, t125290: f64, t125298: f64, t125305: f64, t125309: f64, t125313: f64, t125314: f64, t125337: f64, t1470: f64, t32145: f64, t32798: f64, t32802: f64, t33359: f64, t33363: f64, t33612: f64, t33617: f64, t36: f64, t606: f64, t7574: f64, t8142: f64, t8442: f64, t8621: f64, t8912: f64, t8913: f64) -> (f64, f64) {
    let t131234 = t34873 * t116;
    let t131256 = t13272 * t33362;
    let t131276 = -5.0_f64 / 18.0_f64 * t124463 * t125269 - 5.0_f64 / 18.0_f64 * t32802 * t8442 * t8142 * t36 * t606 - 5.0_f64 / 18.0_f64 * t124463 * t125280 + 5.0_f64 / 6.0_f64 * t124456 * t125337 + 5.0_f64 / 6.0_f64 * t124456 * t125261 - 5.0_f64 / 18.0_f64 * t32802 * t8442 * t1470 * t7574 + 5.0_f64 / 144.0_f64 * t125283 * t8913 - 5.0_f64 / 36.0_f64 * t131256 * t32145 + 35.0_f64 / 24.0_f64 * t124483 * t125290 - 5.0_f64 / 12.0_f64 * t32798 * t8621 * t33612 * t7574 - 5.0_f64 / 12.0_f64 * t33359 * t125298 - 5.0_f64 / 36.0_f64 * t124480 * t33617 - 5.0_f64 / 36.0_f64 * t33363 * t125305 - 5.0_f64 / 36.0_f64 * t33363 * t125309 + 5.0_f64 / 18.0_f64 * t125313 * t8912 * t125314;
    (t131234, t131276)
}
