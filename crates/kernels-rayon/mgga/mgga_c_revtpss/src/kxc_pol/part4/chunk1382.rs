//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1382/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1382(t17483: f64, t3720: f64, t1222: f64, t12809: f64, t12882: f64, t12887: f64, t12893: f64, t12895: f64, t12900: f64, t12902: f64, t12905: f64, t17474: f64, t17476: f64, t17479: f64) -> f64 {
    let t17484 = t3720 * t17483;
    let t17493 = t17474 - 7.0_f64 / 648.0_f64 * t1222 * t17476 + t1222 * t17479 / 36.0_f64 + 0.21437009059034868486e-3_f64 * t12809 * t17484 + 0.63517063878621832551e-4_f64 * t12882 + 0.15879265969655458138e-3_f64 * t12887 - 0.95275595817932748826e-4_f64 * t12893 + 0.14291339372689912324e-3_f64 * t12895 + t12900 + 0.28582678745379824648e-3_f64 * t12902 - 0.95275595817932748826e-4_f64 * t12905;
    t17493
}
