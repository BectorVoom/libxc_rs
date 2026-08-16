//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1078/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1078(t35610: f64, t8392: f64, t1882: f64, t35645: f64, t35574: f64, t35739: f64, t10007: f64, t110401: f64, t110751: f64, t1168: f64, t14127: f64, t14159: f64, t14175: f64, t142009: f64, t142020: f64, t142030: f64, t150924: f64, t150928: f64, t151405: f64, t1901: f64, t242: f64, t2469: f64, t2574: f64, t265: f64, t27986: f64, t28128: f64, t28136: f64, t28368: f64, t33452: f64, t33716: f64, t35516: f64, t35594: f64, t35634: f64, t35737: f64, t3870: f64, t4005: f64, t446: f64, t684: f64, t729: f64, t7440: f64, t762: f64, t766: f64) -> f64 {
    let t151976 = t8392 * t35610;
    let t151985 = t1882 * t35645;
    let t152028 = t1882 * t35574;
    let t152030 = t1882 * t35739;
    let t152032 = -4.0_f64 / 3.0_f64 * t1901 * t110751 * t28368 - 4.0_f64 / 3.0_f64 * t1901 * t110401 * t28136 + 2.0_f64 / 3.0_f64 * t142009 + 4.0_f64 / 9.0_f64 * t151976 + 2.0_f64 / 3.0_f64 * t446 * t2574 * t4005 * t7440 - 2.0_f64 * t446 * t242 * t151405 - 2.0_f64 / 9.0_f64 * t151985 + 2.0_f64 / 3.0_f64 * t446 * t2574 * t265 * t150928 + t446 * t729 * t2469 * t35594 / 3.0_f64 + t446 * t729 * t762 * t33452 * t1168 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t2574 * t265 * t150924 + t446 * t729 * t762 * t35516 * t766 / 3.0_f64 - 4.0_f64 / 9.0_f64 * t1901 * t14175 * t35737 * t684 - t1901 * t10007 * t35634 * t684 / 9.0_f64 + 2.0_f64 / 9.0_f64 * t142020 + t1901 * t142030 * t3870 / 9.0_f64 + t1901 * t14159 * t33716 / 9.0_f64 - 4.0_f64 / 3.0_f64 * t1901 * t14127 * t28128 * t27986 - 4.0_f64 / 9.0_f64 * t152028 - 4.0_f64 / 9.0_f64 * t152030;
    t152032
}
