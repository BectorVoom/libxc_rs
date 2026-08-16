//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2341/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2341(t225: f64, t29665: f64, t8006: f64, t94490: f64, t11606: f64, t1190: f64, t1238: f64, t1252: f64, t15797: f64, t15820: f64, t1716: f64, t1720: f64, t19208: f64, t19213: f64, t19219: f64, t24615: f64, t27721: f64, t27784: f64, t27785: f64, t29536: f64, t29664: f64, t3593: f64, t498: f64, t6243: f64, t7283: f64, t7300: f64, t7301: f64, t7391: f64, t8014: f64, t8061: f64, t8088: f64, t86501: f64, t94391: f64, t94558: f64, t95912: f64) -> f64 {
    let t104635 = t29665 * t225;
    let t104647 = t94490 * t8006;
    let t104669 = 0.16449340668482264365e-1_f64 * t7283 * t1716 * t94391 - t104635 * t1252 + t1190 * t29664 * t498 - 2.0_f64 * t15797 * t8088 - 12.0_f64 * t27784 * t27785 * t19213 + 2.0_f64 * t3593 * t29536 - 0.18277045187202515961e-2_f64 * t86501 + 0.14621636149762012769e-1_f64 * t104647 + 2.0_f64 * t1720 * t27721 * t498 - 0.16449340668482264365e-1_f64 * t7283 * t94558 * t8014 - 6.0_f64 * t1238 * t11606 * t7391 * t6243 + 0.16449340668482264365e-1_f64 * t7283 * t7300 * t24615 * t19219 - 0.82246703342411321825e-2_f64 * t7283 * t7300 * t7301 * t19208 - t95912 + 4.0_f64 * t15820 * t8061;
    t104669
}
