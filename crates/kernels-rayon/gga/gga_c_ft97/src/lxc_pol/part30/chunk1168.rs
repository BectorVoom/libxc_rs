//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1168/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1168(t1882: f64, t36191: f64, t34053: f64, t4246: f64, t36179: f64, t25188: f64, t28930: f64, t112760: f64, t112975: f64, t1212: f64, t144093: f64, t144094: f64, t144096: f64, t144105: f64, t144107: f64, t15133: f64, t152673: f64, t152678: f64, t152722: f64, t15299: f64, t1901: f64, t29189: f64, t296: f64, t33961: f64, t33994: f64, t34118: f64, t34225: f64, t4176: f64, t4181: f64, t4260: f64, t44528: f64, t446: f64, t56643: f64, t72190: f64, t7629: f64, t840: f64) -> (f64, f64, f64) {
    let t154538 = t1882 * t36191;
    let t154550 = t4246 * t34053;
    let t154568 = t1882 * t36179;
    let t154586 = t25188 * t28930;
    let t154590 = 2.0_f64 / 3.0_f64 * t446 * t840 * t4246 * t34118 - 2.0_f64 / 9.0_f64 * t154538 - t144093 - 2.0_f64 / 9.0_f64 * t144094 - 2.0_f64 / 9.0_f64 * t144096 + 8.0_f64 / 3.0_f64 * t1901 * t72190 * t7629 * t4176 + 4.0_f64 * t1901 * t112975 * t7629 * t4181 - t446 * t296 * t154550 / 3.0_f64 + t144105 / 9.0_f64 + t144107 / 9.0_f64 - t446 * t840 * t33994 * t1212 / 3.0_f64 + t446 * t840 * t4246 * t34225 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t840 * t15133 * t7629 + 2.0_f64 / 9.0_f64 * t154568 + 2.0_f64 / 9.0_f64 * t1901 * t44528 * t33961 * t4260 - 4.0_f64 / 9.0_f64 * t1901 * t15299 * t152722 + 4.0_f64 / 9.0_f64 * t1901 * t15299 * t152673 - 4.0_f64 / 27.0_f64 * t1901 * t56643 * t152678 - 4.0_f64 / 9.0_f64 * t1901 * t112760 * t29189 + 4.0_f64 / 3.0_f64 * t446 * t296 * t154586;
    (t154550, t154586, t154590)
}
