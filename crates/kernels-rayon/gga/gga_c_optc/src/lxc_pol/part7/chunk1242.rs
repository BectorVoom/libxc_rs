//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1242/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1242(t11368: f64, t11451: f64, t11455: f64, t18634: f64, t23825: f64, t25063: f64, t25117: f64, t25365: f64, t25445: f64, t25610: f64, t25618: f64, t25620: f64, t25624: f64, t25633: f64, t2591: f64, t2598: f64, t2721: f64, t2722: f64, t2812: f64, t3836: f64, t3917: f64, t3918: f64, t7870: f64, t8187: f64, t8203: f64, t8220: f64, t894: f64, t914: f64, t930: f64, t953: f64) -> f64 {
    let t25646 = -0.28131159491972598279e5_f64 * t11451 * t8203 + 0.14065579745986299139e5_f64 * t11455 * t8187 + 0.35163949364965747848e4_f64 * t3917 * t25610 * t3918 - 0.90880810212048753088e1_f64 * t11368 * t18634 * t25445 + 0.1559479530529405812e2_f64 * t25618 - 0.80782942410710002746e1_f64 * t25620 - 0.10097867801338750343e1_f64 * t25624 + 0.15146801702008125515e1_f64 * t2721 * t2722 * t25365 - 0.30228422675018518374e0_f64 * t953 * t894 * t7870 * t23825 - 0.1559479530529405812e3_f64 * t2812 * t3836 * t25633 + 0.10210489436895143984e1_f64 * t8220 * t2591 + 0.17017482394825239973e1_f64 * t8220 * t2598 + 0.11590881986385010473e0_f64 * t930 * t914 * t25117 + 0.25190352229182098644e-1_f64 * t953 * t25063;
    t25646
}
