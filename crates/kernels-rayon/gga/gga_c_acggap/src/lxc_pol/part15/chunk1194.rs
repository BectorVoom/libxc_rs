//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1194/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1194(t2176: f64, t5511: f64, t39743: f64, t7942: f64, t8306: f64, t2146: f64, t2395: f64, t32329: f64, t32990: f64, t32997: f64, t33000: f64, t33566: f64, t38008: f64, t38010: f64, t38015: f64, t38018: f64, t38019: f64, t38092: f64, t463: f64, t7912: f64, t7931: f64, t8004: f64, t8440: f64, t9982: f64, t9985: f64) -> f64 {
    let t41044 = t2176 * t5511;
    let t41055 = t7942 * t8306 * t39743;
    let t41065 = -0.34694512752820797848e1_f64 * t38008 + 0.65854491829355115987e0_f64 * t41044 - 0.17347256376410398924e1_f64 * t38010 - 0.8673628188205199462e0_f64 * t32329 + t38015 - 0.17347256376410398924e1_f64 * t7931 * t38092 * t8440 - 0.8673628188205199462e0_f64 * t7912 * t9982 + t38018 + 0.26341796731742046394e1_f64 * t38019 - 0.8673628188205199462e0_f64 * t41055 - 0.17347256376410398924e1_f64 * t32990 + t32997 + 0.52041769129231196772e1_f64 * t33000 + 0.17347256376410398924e1_f64 * t33566 * t2395 - 0.52041769129231196772e1_f64 * t2146 * t8004 * t9985 * t463;
    t41065
}
