//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 954/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk954(t5: f64, t2247: f64, t32148: f64, t6972: f64, t8441: f64, t8621: f64, t32135: f64, t640: f64, t84: f64, t32132: f64, t32136: f64, t32138: f64, t32142: f64, t32145: f64, t8443: f64) -> (f64, f64, f64, f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t32149 = t2247 * t32148;
    let t32151 = t8621 * t8441 * t6972;
    let t32154 = t2247 * t32135;
    let t32156 = t8621 * t84 * t640;
    let t32160 = piecewise3(t8, 0.0_f64, 5.0_f64 / 144.0_f64 * t32132 * t8443 - 5.0_f64 / 24.0_f64 * t32136 * t32138 - 5.0_f64 / 36.0_f64 * t32142 * t32145 + 5.0_f64 / 72.0_f64 * t32149 * t32151 + 5.0_f64 / 72.0_f64 * t32154 * t32156);
    (t32149, t32151, t32154, t32156, t32160)
}
