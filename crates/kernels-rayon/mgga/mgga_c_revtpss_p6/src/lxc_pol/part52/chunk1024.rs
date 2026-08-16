//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1024/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1024(t32177: f64, t27: f64, t8571: f64, t221: f64, t4019: f64, t561: f64, t786: f64, t7063: f64, t1385: f64, t239: f64) -> (f64, f64, f64, f64, f64) {
    let t32178 = 2.0_f64 * t32177;
    let t32183 = t8571 * t27;
    let t32186 = t4019 * t221 * t561;
    let t32187 = t786 * t32183 * t32186;
    let t32190 = t7063 * t32183 * t32186;
    let t32192 = t1385 * t239;
    (t32178, t32186, t32187, t32190, t32192)
}
