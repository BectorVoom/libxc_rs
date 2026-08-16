//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1306/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1306(t11889: f64, t1193: f64, t13930: f64, t3207: f64, t39689: f64, t53610: f64, t53626: f64, t53629: f64, t53636: f64, t53761: f64, t54545: f64, t56657: f64, t56661: f64, t56667: f64, t56671: f64, t56674: f64, t56678: f64, t56686: f64, t827: f64, t8793: f64, t9283: f64) -> f64 {
    let t56694 = t56657 / 768.0_f64 - t827 * t56661 / 48.0_f64 + t56667 / 384.0_f64 - t827 * t56671 / 96.0_f64 - t56674 / 48.0_f64 - t56678 / 384.0_f64 - t53610 + t8793 * t53761 / 24.0_f64 + t8793 * t54545 / 24.0_f64 - t56686 / 1536.0_f64 + t39689 * t13930 / 48.0_f64 + t53626 + t53629 - t3207 * t9283 * t1193 * t11889 / 8.0_f64 + t53636;
    t56694
}
