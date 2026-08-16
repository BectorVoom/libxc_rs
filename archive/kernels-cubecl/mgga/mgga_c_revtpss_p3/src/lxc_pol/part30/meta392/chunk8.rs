//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1473/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1473<F: Float>(t14151: F, t14200: F, t14237: F, t14266: F, t1427: F, t1904: F, t3899: F, t689: F, t10151: F, t10154: F, t14091: F, t14096: F, t14097: F, t14102: F, t14105: F, t14108: F, t14111: F, t1424: F, t4132: F, t5715: F, t9695: F) -> (F, F, F) {
    let t14268 = t14151 + t14200 + t14237 + t14266;
    let t14269 = t1427 * t14268;
    let t14274 = t3899 * t1904;
    let t14276 = F::cast_from(0.10975748638225852664e-1_f64) * t689 * t14274;
    let t14279 = F::cast_from(0.13009920719177044025e-1_f64) * t14091 - F::cast_from(0.2601984143835408805e-1_f64) * t9695 + t14096 + F::cast_from(0.73171657588172351096e-2_f64) * t14097 - t14102 - F::cast_from(0.11565819519348392139e-2_f64) * t14105 - t14108 + F::cast_from(0.39029762157531132075e-1_f64) * t14111 - F::cast_from(0.65854491829355115987e0_f64) * t1424 * t14269 - F::cast_from(0.65854491829355115987e0_f64) * t5715 * t4132 + t14276 - F::cast_from(0.10975748638225852664e-1_f64) * t10151 + F::cast_from(0.10975748638225852664e-1_f64) * t10154;
    (t14268, t14269, t14279)
}
