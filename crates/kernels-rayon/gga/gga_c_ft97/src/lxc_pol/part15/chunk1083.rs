//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1083/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1083(t87252: f64, t87285: f64, t1017: f64, t20022: f64, t1053: f64, t1060: f64, t12714: f64, t13212: f64, t1901: f64, t20023: f64, t20045: f64, t20660: f64, t20709: f64, t20743: f64, t20763: f64, t2179: f64, t2185: f64, t2205: f64, t2983: f64, t3578: f64, t4454: f64, t446: f64, t4462: f64, t4668: f64, t4714: f64, t4724: f64, t4733: f64, t4805: f64, t4839: f64, t50773: f64, t569: f64, t574: f64, t605: f64, t86977: f64, t9144: f64, t9327: f64, t9432: f64) -> (f64, f64, f64, f64) {
    let t87286 = t87252 + t87285;
    let t87295 = t20022 * t1017;
    let t87303 = t20022 * t1053;
    let t87372 = -8.0_f64 * t446 * t2185 * t3578 * t20709 - 4.0_f64 * t446 * t2185 * t605 * t4668 * t4805 + 8.0_f64 * t446 * t9432 * t605 * t20660 * t1053 - 4.0_f64 / 3.0_f64 * t1901 * t9144 * t4462 * t4733 + 8.0_f64 / 9.0_f64 * t1901 * t12714 * t2983 * t20763 + 8.0_f64 / 9.0_f64 * t1901 * t13212 * t86977 - 8.0_f64 / 3.0_f64 * t1901 * t50773 * t20743 - 4.0_f64 / 9.0_f64 * t446 * t569 * t1060 * t20045 - 4.0_f64 / 9.0_f64 * t446 * t2205 * t4839 * t4454 - 4.0_f64 * t446 * t574 * t2179 * t4714 * t4724 - 40.0_f64 / 81.0_f64 * t446 * t9327 * t1060 * t20023;
    (t87286, t87295, t87303, t87372)
}
