//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1055/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1055(t136: f64, t7342: f64, t2247: f64, t1925: f64, t36: f64, t606: f64, t8442: f64, t624: f64, t8435: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t32589 = t7342 * t136;
    let t32590 = t2247 * t32589;
    let t32591 = t1925 * t36;
    let t32592 = t32591 * t606;
    let t32593 = t8442 * t32592;
    let t32596 = t8435 * t624;
    let t32597 = t2247 * t32596;
    (t32589, t32590, t32591, t32593, t32596, t32597)
}
