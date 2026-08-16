//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2736/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2736(t10489: f64, t10639: f64, t14676: f64, t231: f64, t2646: f64, t2745: f64, t2747: f64, t40333: f64, t40337: f64, t40345: f64, t40349: f64, t40355: f64, t40357: f64, t40361: f64, t40365: f64, t4364: f64, t4365: f64, t4450: f64, t50292: f64, t50296: f64, t50299: f64, t50303: f64, t50308: f64, t50312: f64, t50325: f64) -> f64 {
    let t50327 = -0.12705000702321332056e-4_f64 * t40333 - 0.8131200449485652516e-3_f64 * t40337 - 0.60023625365297631762e-2_f64 * t40345 + 0.60023625365297631762e-2_f64 * t40349 - 0.76230004213927992336e-4_f64 * t40355 + 0.40656002247428262581e-3_f64 * t40357 + 0.11337795902333997111e0_f64 * t40361 + 0.85748036236139473944e-3_f64 * t2745 * t2747 * t4450 * t10639 + 0.15246000842785598468e-3_f64 * t50292 + 0.18007087609589289528e-1_f64 * t50296 - t50299 - 0.12705000702321332056e-4_f64 * t40365 - 0.15246000842785598468e-3_f64 * t50303 + 0.21437009059034868486e-4_f64 * t50308 - 0.42874018118069736972e-3_f64 * t50312 - 0.64311027177104605458e-3_f64 * t2745 * t4364 * t14676 * t2646 + 0.85748036236139473944e-3_f64 * t2745 * t2747 * t4365 * t231 * t10489 + 0.15246000842785598468e-3_f64 * t50325;
    t50327
}
