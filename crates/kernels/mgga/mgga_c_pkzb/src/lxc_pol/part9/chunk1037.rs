//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1037/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1037<F: Float>(t8559: F, t942: F, t1246: F, t1256: F, t2422: F, t2430: F, t2454: F, t3247: F, t3255: F, t3279: F, t411: F, t415: F, t8481: F, t8497: F, t8501: F, t8504: F, t938: F, t952: F) -> (F, F) {
    let t8560 = t942 * t8559;
    let t8563 = F::cast_from(0.65854491829355115987e0_f64) * t8481 * t415 - F::cast_from(0.13170898365871023197e1_f64) * t3247 * t952 + F::cast_from(0.13170898365871023197e1_f64) * t1246 * t2430 - F::cast_from(0.65854491829355115987e0_f64) * t1246 * t2454 - F::cast_from(0.65854491829355115987e0_f64) * t2422 * t1256 + F::cast_from(0.26341796731742046394e1_f64) * t938 * t3255 - F::cast_from(0.13170898365871023197e1_f64) * t938 * t3279 - F::cast_from(0.39512695097613069591e1_f64) * t411 * t8497 + F::cast_from(0.26341796731742046394e1_f64) * t411 * t8501 + F::cast_from(0.13170898365871023197e1_f64) * t411 * t8504 - F::cast_from(0.65854491829355115987e0_f64) * t411 * t8560;
    (t8560, t8563)
}
