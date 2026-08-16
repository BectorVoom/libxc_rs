//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3253/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3253(t18498: f64, t221: f64, t10703: f64, t2674: f64, t14468: f64, t1544: f64, t231: f64, t2477: f64, t2745: f64, t2747: f64, t4365: f64, t50436: f64, t50443: f64, t50453: f64, t50457: f64, t50466: f64, t61697: f64, t61699: f64, t61701: f64, t61718: f64, t828: f64, t837: f64, t851: f64) -> f64 {
    let t61725 = t221 * t18498;
    let t61727 = t2674 * t10703 * t61725;
    let t61730 = -0.50820002809285328226e-3_f64 * t61697 + 0.45351183609335988442e-1_f64 * t61699 + 0.17149607247227894789e-2_f64 * t2745 * t2747 * t61701 * t837 - 0.40164115440237189888e-6_f64 * t50436 + 0.60976381323476959249e-3_f64 * t50443 + 0.17149607247227894789e-2_f64 * t2745 * t2747 * t4365 * t231 * t14468 - 0.4065600224742826258e-4_f64 * t50453 + 0.36143185997963725434e-3_f64 * t50457 + 0.50820002809285328225e-4_f64 * t61718 + 0.85748036236139473944e-2_f64 * t851 * t2477 * t828 * t1544 * t14468 + 0.10164000561857065645e-2_f64 * t61727 - 0.50820002809285328225e-3_f64 * t50466;
    t61730
}
