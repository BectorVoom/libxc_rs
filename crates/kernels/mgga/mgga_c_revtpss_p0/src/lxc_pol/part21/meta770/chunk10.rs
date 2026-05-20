//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2736/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2736<F: Float>(t10489: F, t10639: F, t14676: F, t231: F, t2646: F, t2745: F, t2747: F, t40333: F, t40337: F, t40345: F, t40349: F, t40355: F, t40357: F, t40361: F, t40365: F, t4364: F, t4365: F, t4450: F, t50292: F, t50296: F, t50299: F, t50303: F, t50308: F, t50312: F, t50325: F) -> F {
    let t50327 = -F::cast_from(0.12705000702321332056e-4_f64) * t40333 - F::cast_from(0.8131200449485652516e-3_f64) * t40337 - F::cast_from(0.60023625365297631762e-2_f64) * t40345 + F::cast_from(0.60023625365297631762e-2_f64) * t40349 - F::cast_from(0.76230004213927992336e-4_f64) * t40355 + F::cast_from(0.40656002247428262581e-3_f64) * t40357 + F::cast_from(0.11337795902333997111e0_f64) * t40361 + F::cast_from(0.85748036236139473944e-3_f64) * t2745 * t2747 * t4450 * t10639 + F::cast_from(0.15246000842785598468e-3_f64) * t50292 + F::cast_from(0.18007087609589289528e-1_f64) * t50296 - t50299 - F::cast_from(0.12705000702321332056e-4_f64) * t40365 - F::cast_from(0.15246000842785598468e-3_f64) * t50303 + F::cast_from(0.21437009059034868486e-4_f64) * t50308 - F::cast_from(0.42874018118069736972e-3_f64) * t50312 - F::cast_from(0.64311027177104605458e-3_f64) * t2745 * t4364 * t14676 * t2646 + F::cast_from(0.85748036236139473944e-3_f64) * t2745 * t2747 * t4365 * t231 * t10489 + F::cast_from(0.15246000842785598468e-3_f64) * t50325;
    t50327
}
