//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1548/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1548(t10817: f64, t4359: f64, t10655: f64, t4400: f64, t4396: f64, t912: f64, t2792: f64, t1557: f64, t2836: f64, t2793: f64, t4399: f64, t10661: f64) -> (f64, f64, f64, f64, f64) {
    let t14376 = 4.0_f64 * t10817 * t4359;
    let t14378 = 0.32163958997385070134e2_f64 * t10655 * t4400;
    let t14379 = t4396 * t912;
    let t14381 = 4.0_f64 * t2792 * t14379;
    let t14382 = t1557 * t2836;
    let t14384 = 2.0_f64 * t2792 * t14382;
    let t14385 = t4399 * t2793;
    let t14387 = 0.96491876992155210402e2_f64 * t10661 * t14385;
    (t14376, t14378, t14381, t14384, t14387)
}
