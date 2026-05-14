//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1411/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1411<F: Float>(t3278: F, t10283: F, t10296: F, t10306: F, t1246: F, t19227: F, t2422: F, t2428: F, t2429: F, t2453: F, t2454: F, t26960: F, t28438: F, t28476: F, t28516: F, t3247: F, t3279: F, t3904: F, t3909: F, t3910: F, t3928: F, t3929: F, t411: F, t6546: F, t8497: F, t8501: F, t938: F, t942: F, t952: F) -> (F,) {
    let t28532 = t3278 * t3278;
    let t28536 = -0.65854491829355115987e0 * t3904 * t2454 - 0.65854491829355115987e0 * t2422 * t3929 + 0.52683593463484092788e1 * t1246 * t8501 - 0.39512695097613069591e1 * t411 * t10296 * t2453 - 0.26341796731742046394e1 * t3247 * t3279 - 0.39512695097613069591e1 * t411 * t6546 * t3928 * t2429 - 0.13170898365871023197e1 * t10283 * t952 - 0.65854491829355115987e0 * t411 * t942 * (t26960 + t28438 + t28476 + t28516) + 0.26341796731742046394e1 * t938 * t10306 + 0.15805078039045227836e2 * t411 * t19227 * t3909 * t2429 - 0.79025390195226139182e1 * t1246 * t8497 + 0.13170898365871023197e1 * t2422 * t3910 + 0.26341796731742046394e1 * t411 * t2428 * t28532;
    (t28536,)
}
