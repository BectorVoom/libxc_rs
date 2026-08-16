//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1319/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1319(t1484: f64, t5611: f64, t13222: f64, t13350: f64, t1510: f64, t16891: f64, t20947: f64, t20972: f64, t20993: f64, t210: f64, t2571: f64, t2643: f64, t46876: f64, t5544: f64, t5567: f64, t58723: f64, t58744: f64, t67880: f64, t67882: f64, t67884: f64, t67920: f64, t67937: f64, t9559: f64, t9646: f64) -> (f64, f64) {
    let t76250 = t1484 * t5611;
    let t76259 = -3.0_f64 / 2.0_f64 * t9559 * t210 * t5567 * t5544 + t2571 * t210 * t20993 * t1484 / 4.0_f64 - 7.0_f64 / 96.0_f64 * t67880 - 7.0_f64 / 1152.0_f64 * t67882 + 7.0_f64 / 1152.0_f64 * t67884 - 5.0_f64 / 64.0_f64 * t2643 * t13350 * t1510 * t20947 - 119.0_f64 / 2304.0_f64 * t58723 + 7.0_f64 / 36.0_f64 * t67920 + 595.0_f64 / 2592.0_f64 * t46876 + 7.0_f64 / 3.0_f64 * t67937 + 35.0_f64 / 12.0_f64 * t58744 + t2643 * t13222 * t1510 * t76250 / 64.0_f64 - 5.0_f64 / 128.0_f64 * t2643 * t9646 * t16891 * t20972;
    (t76250, t76259)
}
