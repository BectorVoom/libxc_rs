//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 658/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk658(t3: f64, t5363: f64, t112: f64, t1851: f64, t1458: f64, t671: f64, t1401: f64, t3938: f64, t3941: f64, t4072: f64, t577: f64, t2235: f64, t33: f64) -> (f64, f64, f64, f64, f64) {
    let t5364 = t3 * t5363;
    let t5371 = t1851 * t112;
    let t5376 = t1458 * t671;
    let t5381 = 0.45e1_f64 * t5363 * t577 + 0.135e2_f64 * t5371 * t671 + 0.135e2_f64 * t3938 * t1458 + 27.0_f64 * t3941 * t5376 + 0.135e2_f64 * t1401 * t4072;
    let t6486 = t2235 * t33;
    (t5364, t5371, t5376, t5381, t6486)
}
