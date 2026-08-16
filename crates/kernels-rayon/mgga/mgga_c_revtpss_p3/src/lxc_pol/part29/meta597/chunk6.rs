//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2021/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2021(t102993: f64, t25411: f64, t103382: f64, t103391: f64, t103393: f64, t103394: f64, t103396: f64, t103399: f64, t103400: f64, t231: f64, t25383: f64, t26547: f64, t28340: f64, t28418: f64, t4534: f64, t7070: f64, t7071: f64, t7076: f64, t836: f64, t886: f64, t95859: f64, t95862: f64, t95866: f64) -> f64 {
    let t103404 = t25411 * t102993;
    let t103412 = t103382 + 0.34270468708064099208e-1_f64 * t95859 - t95862 + 0.9757440539382783019e-2_f64 * t95866 + 0.8673628188205199462e0_f64 * t7070 * t7076 * t28340 * t836 * t231 - t103391 + t103393 - 0.22849835011101738147e-2_f64 * t103394 + 0.39029762157531132075e-1_f64 * t103396 - t103399 - 0.73171657588172351096e-2_f64 * t103400 - 0.13170898365871023197e1_f64 * t26547 * t4534 - 0.17135234354032049604e-1_f64 * t103404 + 0.17347256376410398924e1_f64 * t25383 * t28418 + 0.17347256376410398924e1_f64 * t7070 * t7071 * t28340 * t886;
    t103412
}
