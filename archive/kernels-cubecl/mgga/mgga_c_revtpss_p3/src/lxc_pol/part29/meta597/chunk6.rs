//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2021/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2021<F: Float>(t102993: F, t25411: F, t103382: F, t103391: F, t103393: F, t103394: F, t103396: F, t103399: F, t103400: F, t231: F, t25383: F, t26547: F, t28340: F, t28418: F, t4534: F, t7070: F, t7071: F, t7076: F, t836: F, t886: F, t95859: F, t95862: F, t95866: F) -> F {
    let t103404 = t25411 * t102993;
    let t103412 = t103382 + F::cast_from(0.34270468708064099208e-1_f64) * t95859 - t95862 + F::cast_from(0.9757440539382783019e-2_f64) * t95866 + F::cast_from(0.8673628188205199462e0_f64) * t7070 * t7076 * t28340 * t836 * t231 - t103391 + t103393 - F::cast_from(0.22849835011101738147e-2_f64) * t103394 + F::cast_from(0.39029762157531132075e-1_f64) * t103396 - t103399 - F::cast_from(0.73171657588172351096e-2_f64) * t103400 - F::cast_from(0.13170898365871023197e1_f64) * t26547 * t4534 - F::cast_from(0.17135234354032049604e-1_f64) * t103404 + F::cast_from(0.17347256376410398924e1_f64) * t25383 * t28418 + F::cast_from(0.17347256376410398924e1_f64) * t7070 * t7071 * t28340 * t886;
    t103412
}
