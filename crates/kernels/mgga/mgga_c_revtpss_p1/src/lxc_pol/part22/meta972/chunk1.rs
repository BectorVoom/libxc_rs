//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3253/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3253<F: Float>(t18498: F, t221: F, t10703: F, t2674: F, t14468: F, t1544: F, t231: F, t2477: F, t2745: F, t2747: F, t4365: F, t50436: F, t50443: F, t50453: F, t50457: F, t50466: F, t61697: F, t61699: F, t61701: F, t61718: F, t828: F, t837: F, t851: F) -> F {
    let t61725 = t221 * t18498;
    let t61727 = t2674 * t10703 * t61725;
    let t61730 = -F::cast_from(0.50820002809285328226e-3_f64) * t61697 + F::cast_from(0.45351183609335988442e-1_f64) * t61699 + F::cast_from(0.17149607247227894789e-2_f64) * t2745 * t2747 * t61701 * t837 - F::cast_from(0.40164115440237189888e-6_f64) * t50436 + F::cast_from(0.60976381323476959249e-3_f64) * t50443 + F::cast_from(0.17149607247227894789e-2_f64) * t2745 * t2747 * t4365 * t231 * t14468 - F::cast_from(0.4065600224742826258e-4_f64) * t50453 + F::cast_from(0.36143185997963725434e-3_f64) * t50457 + F::cast_from(0.50820002809285328225e-4_f64) * t61718 + F::cast_from(0.85748036236139473944e-2_f64) * t851 * t2477 * t828 * t1544 * t14468 + F::cast_from(0.10164000561857065645e-2_f64) * t61727 - F::cast_from(0.50820002809285328225e-3_f64) * t50466;
    t61730
}
