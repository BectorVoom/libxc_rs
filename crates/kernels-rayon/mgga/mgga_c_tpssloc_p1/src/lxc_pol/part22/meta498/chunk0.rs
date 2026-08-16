//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1929/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1929(t10996: f64, t20234: f64, t974: f64, t1616: f64, t5685: f64, t3071: f64, t5677: f64, t10408: f64, t1539: f64, t5867: f64, t21118: f64, t248: f64, t3062: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t21561 = t10996 * t20234;
    let t21562 = t974 * t21561;
    let t21565 = t5685 * t1616;
    let t21566 = t3071 * t21565;
    let t21569 = t5677 * t1616;
    let t21570 = t10408 * t21569;
    let t21573 = t5867 * t1539;
    let t21574 = t3071 * t21573;
    let t21580 = t248 * t3062 * t21118;
    (t21561, t21562, t21565, t21566, t21569, t21570, t21573, t21574, t21580)
}
