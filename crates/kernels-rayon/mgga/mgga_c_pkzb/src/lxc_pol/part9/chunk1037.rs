//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1037/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1037(t8559: f64, t942: f64, t1246: f64, t1256: f64, t2422: f64, t2430: f64, t2454: f64, t3247: f64, t3255: f64, t3279: f64, t411: f64, t415: f64, t8481: f64, t8497: f64, t8501: f64, t8504: f64, t938: f64, t952: f64) -> (f64, f64) {
    let t8560 = t942 * t8559;
    let t8563 = 0.65854491829355115987e0_f64 * t8481 * t415 - 0.13170898365871023197e1_f64 * t3247 * t952 + 0.13170898365871023197e1_f64 * t1246 * t2430 - 0.65854491829355115987e0_f64 * t1246 * t2454 - 0.65854491829355115987e0_f64 * t2422 * t1256 + 0.26341796731742046394e1_f64 * t938 * t3255 - 0.13170898365871023197e1_f64 * t938 * t3279 - 0.39512695097613069591e1_f64 * t411 * t8497 + 0.26341796731742046394e1_f64 * t411 * t8501 + 0.13170898365871023197e1_f64 * t411 * t8504 - 0.65854491829355115987e0_f64 * t411 * t8560;
    (t8560, t8563)
}
