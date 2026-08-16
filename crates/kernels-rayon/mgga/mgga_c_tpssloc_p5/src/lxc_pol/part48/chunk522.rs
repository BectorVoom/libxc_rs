//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 522/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk522(t1401: f64, t2319: f64, t2363: f64, t3931: f64, t3938: f64, t3941: f64, t577: f64, t671: f64, t89: f64) -> (f64, f64) {
    let t3946 = 0.45e1_f64 * t3931 * t577 + 27.0_f64 * t3938 * t671 + 27.0_f64 * t3941 * t2319 + 0.135e2_f64 * t1401 * t2363;
    let t4034 = t89 * t671;
    (t3946, t4034)
}
