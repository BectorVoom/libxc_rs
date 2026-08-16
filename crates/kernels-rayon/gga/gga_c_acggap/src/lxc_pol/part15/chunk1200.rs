//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1200/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1200(t39499: f64, t7942: f64, t8306: f64, t38226: f64, t557: f64, t310: f64, t9973: f64, t40703: f64, t7963: f64, t1938: f64, t8331: f64, t2146: f64, t30023: f64, t33150: f64, t33153: f64, t33157: f64, t33778: f64, t38215: f64, t38224: f64, t38662: f64, t40749: f64, t463: f64, t7931: f64, t9003: f64, t9145: f64, t9162: f64, t9402: f64, t9976: f64) -> f64 {
    let t41196 = t7942 * t8306 * t39499;
    let t41200 = t38226 * t557;
    let t41211 = t310 * t9973;
    let t41214 = t7963 * t8306 * t40703;
    let t41216 = t8331 * t1938;
    let t41225 = -0.8673628188205199462e0_f64 * t41196 - 0.17347256376410398924e1_f64 * t33778 * t9162 - 0.13170898365871023197e1_f64 * t41200 + 0.8673628188205199462e0_f64 * t9003 * t9145 + 0.52041769129231196772e1_f64 * t38215 - 0.17347256376410398924e1_f64 * t7931 * t8306 * t40749 - 0.69389025505641595696e1_f64 * t33150 + 0.26020884564615598386e1_f64 * t33153 + 0.34694512752820797848e1_f64 * t33157 + 0.65854491829355115987e0_f64 * t41211 + 0.8673628188205199462e0_f64 * t41214 - 0.65854491829355115987e0_f64 * t41216 + 0.10408353825846239354e2_f64 * t2146 * t30023 * t9976 * t463 + 0.8673628188205199462e0_f64 * t38662 * t9402 - 0.69389025505641595696e1_f64 * t38224;
    t41225
}
