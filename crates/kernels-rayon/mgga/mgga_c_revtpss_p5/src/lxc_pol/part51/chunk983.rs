//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 983/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk983(t5: f64, t33620: f64, t8621: f64, t1493: f64, t84: f64, t32136: f64, t32142: f64, t32149: f64, t32154: f64, t33609: f64, t33613: f64, t33617: f64, t8443: f64) -> (f64, f64, f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t33621 = t8621 * t33620;
    let t33624 = t84 * t1493;
    let t33625 = t8621 * t33624;
    let t33629 = piecewise3(t8, 0.0_f64, 5.0_f64 / 144.0_f64 * t33609 * t8443 - 5.0_f64 / 24.0_f64 * t32136 * t33613 - 5.0_f64 / 36.0_f64 * t32142 * t33617 + 5.0_f64 / 72.0_f64 * t32149 * t33621 + 5.0_f64 / 72.0_f64 * t32154 * t33625);
    (t33621, t33624, t33625, t33629)
}
