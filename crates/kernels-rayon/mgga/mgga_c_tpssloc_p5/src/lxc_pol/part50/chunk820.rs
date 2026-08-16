//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 820/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk820(t1052: f64, t1920: f64, t1956: f64, t388: f64, t6687: f64, t6771: f64, t8377: f64, t8381: f64, t8392: f64, t8397: f64, t8407: f64, t1958: f64) -> (f64, f64) {
    let t8409 = 0.16449340668482264365e-1_f64 * t1920 * t8377 - 0.16449340668482264365e-1_f64 * t6687 * t8381 + t8392 * t388 - 2.0_f64 * t6771 * t1956 + 2.0_f64 * t1052 * t8397 - t1052 * t8407;
    let t8413 = t1958 * t1958;
    (t8409, t8413)
}
