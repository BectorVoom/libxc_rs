//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 834/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk834<F: Float>(t3361: F, t635: F, t57: F, t268: F, t404: F, t7021: F, t159: F, t3617: F, t409: F, t416: F, t406: F, t11335: F, t281: F, t414: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12256 = F::cast_from(1.0_f64) / t3361 / t635;
    let t12267 = t3361 * t57;
    let t12268 = F::cast_from(1.0_f64) / t12267;
    let t12295 = t268 * t7021 * t404;
    let t12296 = F::cast_from(28.0_f64) / F::cast_from(27.0_f64) * t12295;
    let t12305 = t159 * t3617;
    let t12327 = F::cast_from(1.0_f64) / t409 / t416 / F::cast_from(4.0_f64);
    let t12331 = F::cast_from(1.0_f64)/pow_3_2::<F>(t406);
    let t12349 = F::cast_from(0.93011851851851851854e0_f64) * t12295;
    let t12351 = t281 * t11335 * t414;
    (t12256, t12268, t12295, t12296, t12305, t12327, t12331, t12349, t12351)
}
