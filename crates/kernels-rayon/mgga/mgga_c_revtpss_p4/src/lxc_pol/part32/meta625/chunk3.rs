//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1981/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1981(t102386: f64, t102396: f64, t102404: f64, t102409: f64, t102411: f64, t102422: f64, t102594: f64, t102656: f64, t108259: f64, t1904: f64, t25921: f64, t25930: f64, t26304: f64, t27864: f64, t27972: f64, t30257: f64, t30309: f64, t6918: f64, t7295: f64, t7296: f64, t7506: f64, t96410: f64, t96412: f64) -> f64 {
    let t109681 = -0.13170898365871023197e1_f64 * t102594 * t1904 + 0.45699670022203476294e-2_f64 * t102386 - 0.73171657588172351096e-2_f64 * t96410 + 0.17135234354032049604e-1_f64 * t96412 + 0.8673628188205199462e0_f64 * t7295 * t7296 * t7506 * t6918 - t102396 + 0.8673628188205199462e0_f64 * t25921 * t30309 + 0.8673628188205199462e0_f64 * t25921 * t30257 - t102404 - 0.19274729307122665472e-1_f64 * t102409 + 0.34270468708064099208e-1_f64 * t102411 - 0.17347256376410398924e1_f64 * t25930 * t26304 * t108259 - 0.17347256376410398924e1_f64 * t25930 * t102656 * t27864 - 0.17347256376410398924e1_f64 * t25930 * t102656 * t27972 - t102422;
    t109681
}
