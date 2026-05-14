//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 939/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk939<F: Float>(t35797: F, t35799: F, t35814: F, t35816: F, t35827: F, t35837: F, t35844: F, t35848: F, t35850: F, t35909: F, t35911: F, t35913: F, t35915: F, t35917: F, t35919: F, t35926: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t37722 = 0.85748036236139473944e-3 * t35797;
    let t37723 = 0.68598428988911579156e-2 * t35799;
    let t37731 = 0.16006300097412701803e-1 * t35814;
    let t37732 = 0.42874018118069736972e-3 * t35816;
    let t37735 = 0.28582678745379824648e-3 * t35827;
    let t37739 = 0.25724410870841842184e-2 * t35837;
    let t37741 = 0.42874018118069736972e-3 * t35844;
    let t37743 = 0.16809375e0 * t35848;
    let t37744 = 0.1120625e0 * t35850;
    let t37777 = 0.916875e-1 * t35909;
    let t37778 = 0.916875e-1 * t35911;
    let t37779 = 0.61125e-1 * t35913;
    let t37780 = 0.61125e-1 * t35915;
    let t37781 = 0.34299214494455789578e-2 * t35917;
    let t37782 = 0.34299214494455789578e-2 * t35919;
    let t37787 = 0.64025200389650807212e-1 * t35926;
    (t37722, t37723, t37731, t37732, t37735, t37739, t37741, t37743, t37744, t37777, t37778, t37779, t37780, t37781, t37782, t37787)
}
