//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1162/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1162(t3418: f64, t4339: f64, t2798: f64, t7058: f64, t6556: f64, t8060: f64, t2497: f64, t8042: f64, t8057: f64, t10305: f64, t4342: f64, t10301: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t31469 = t4339 * t3418;
    let t31470 = t2798 * t7058;
    let t31472 = 2.0_f64 * t6556 * t8060;
    let t31474 = 2.0_f64 * t8042 * t2497;
    let t31476 = 4.0_f64 * t6556 * t8057;
    let t31478 = 4.0_f64 * t4342 * t10305;
    let t31480 = 4.0_f64 * t4342 * t10301;
    (t31469, t31470, t31472, t31474, t31476, t31478, t31480)
}
