//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 868/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk868<F: Float>(t11116: F, t2924: F, t1077: F, t225: F, t1096: F, t3270: F, t1071: F, t3046: F, t268: F, t271: F, t7021: F) -> (F, F, F, F) {
    let t11118 = F::cast_from(0.48245938496077605201e2_f64) * t2924 * t11116;
    let t11119 = t1077 * t1077;
    let t11120 = F::cast_from(1.0_f64) / t11119;
    let t11121 = t225 * t11120;
    let t11122 = t3270 * t1096;
    let t11123 = t11121 * t11122;
    let t11128 = t3046 * t1071;
    let t11132 = t268 * t7021 * t271;
    (t11118, t11123, t11128, t11132)
}
