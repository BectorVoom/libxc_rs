//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 657/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk657(t1174: f64, t1726: f64, t1737: f64, t3577: f64, t488: f64, t4889: f64, t4957: f64, t4959: f64, t4994: f64, t4998: f64, t5002: f64, t6158: f64, t6165: f64, t6170: f64, t6178: f64, t6184: f64, t6188: f64, t6192: f64) -> f64 {
    let t6197 = -t6158 * t488 / 288.0_f64 + 19.0_f64 / 1728.0_f64 * t6165 * t488 + t6170 * t488 / 3072.0_f64 + t4957 / 2304.0_f64 - t4959 / 432.0_f64 - t4994 / 3456.0_f64 + t4998 / 2304.0_f64 + t1174 * t6178 / 216.0_f64 + t4889 * t1726 / 54.0_f64 - t1174 * t6184 / 288.0_f64 - t1174 * t6188 / 144.0_f64 - t3577 * t6192 / 2304.0_f64 + t5002 * t1737 / 1536.0_f64;
    t6197
}
