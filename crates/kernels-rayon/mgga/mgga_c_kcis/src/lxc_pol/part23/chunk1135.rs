//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1135/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1135(t1363: f64, t16349: f64, t12321: f64, t2011: f64, t12234: f64, t1385: f64, t3751: f64, t4992: f64, t86: f64, t3960: f64, t5623: f64, t1494: f64, t5627: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t51602 = t16349 * t1363;
    let t51613 = t12321 * t2011;
    let t51622 = t12234 * t1385;
    let t51692 = t86 * t4992 * t3751;
    let t51799 = t5623 * t3960;
    let t52073 = t1494 * t5627;
    (t51602, t51613, t51622, t51692, t51799, t52073)
}
