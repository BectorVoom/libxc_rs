//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1981/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1981<F: Float>(t102386: F, t102396: F, t102404: F, t102409: F, t102411: F, t102422: F, t102594: F, t102656: F, t108259: F, t1904: F, t25921: F, t25930: F, t26304: F, t27864: F, t27972: F, t30257: F, t30309: F, t6918: F, t7295: F, t7296: F, t7506: F, t96410: F, t96412: F) -> F {
    let t109681 = -F::cast_from(0.13170898365871023197e1_f64) * t102594 * t1904 + F::cast_from(0.45699670022203476294e-2_f64) * t102386 - F::cast_from(0.73171657588172351096e-2_f64) * t96410 + F::cast_from(0.17135234354032049604e-1_f64) * t96412 + F::cast_from(0.8673628188205199462e0_f64) * t7295 * t7296 * t7506 * t6918 - t102396 + F::cast_from(0.8673628188205199462e0_f64) * t25921 * t30309 + F::cast_from(0.8673628188205199462e0_f64) * t25921 * t30257 - t102404 - F::cast_from(0.19274729307122665472e-1_f64) * t102409 + F::cast_from(0.34270468708064099208e-1_f64) * t102411 - F::cast_from(0.17347256376410398924e1_f64) * t25930 * t26304 * t108259 - F::cast_from(0.17347256376410398924e1_f64) * t25930 * t102656 * t27864 - F::cast_from(0.17347256376410398924e1_f64) * t25930 * t102656 * t27972 - t102422;
    t109681
}
