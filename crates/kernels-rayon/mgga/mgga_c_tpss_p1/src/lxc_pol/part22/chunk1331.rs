//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1331/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1331(t1270: f64, t12810: f64, t18546: f64, t6242: f64, t4466: f64, t60738: f64, t12865: f64, t18454: f64, t12819: f64, t12831: f64, t19476: f64, t13000: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t65501 = t1270 * t12810;
    let t65533 = t6242 * t18546;
    let t65551 = t60738 * t4466;
    let t65553 = t18454 * t12865;
    let t65555 = t18454 * t12819;
    let t65557 = t19476 * t12831;
    let t65559 = t19476 * t13000;
    (t65501, t65533, t65551, t65553, t65555, t65557, t65559)
}
