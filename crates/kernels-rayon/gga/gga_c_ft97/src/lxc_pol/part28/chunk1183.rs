//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1183/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1183(t1349: f64, t27178: f64, t1546: f64, t34988: f64, t1359: f64, t1360: f64, t138568: f64, t1526: f64, t1527: f64, t15567: f64, t2258: f64, t23400: f64, t27030: f64, t27035: f64, t27100: f64, t27103: f64, t27175: f64, t27182: f64, t27186: f64, t28: f64, t2984: f64, t2993: f64, t3000: f64, t3052: f64, t32665: f64, t32670: f64, t3450: f64, t34985: f64, t34989: f64, t5766: f64, t5772: f64, t5922: f64, t6580: f64, t6678: f64, t8633: f64) -> f64 {
    let t149518 = t1349 * t27178;
    let t149524 = t1349 * t1546 * t34988;
    let t149549 = t15567 * t2258 * t1359 * t2993 / 6.0_f64 - t15567 * t8633 * t1359 * t2984 / 9.0_f64 + t1349 * t27175 / 3.0_f64 + t5766 * t6678 / 3.0_f64 + t1349 * t27182 / 3.0_f64 + t1349 * t27186 / 3.0_f64 - t149518 / 9.0_f64 + t6580 * t5922 / 3.0_f64 - t138568 / 54.0_f64 - t149524 / 54.0_f64 - t1526 * t1527 * t27035 / 12.0_f64 - t1526 * t1527 * t27030 / 12.0_f64 - t34985 * t32670 / 6.0_f64 - t1349 * t28 * t23400 * t3450 + t1349 * t3000 * t1360 * t3052 / 9.0_f64 + t6580 * t32665 / 18.0_f64 + t5766 * t34989 / 18.0_f64 - t5772 * t27100 / 9.0_f64 + t5772 * t27103 / 27.0_f64;
    t149549
}
