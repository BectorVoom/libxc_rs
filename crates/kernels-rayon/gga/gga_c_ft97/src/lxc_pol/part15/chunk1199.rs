//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1199/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1199(t10697: f64, t5309: f64, t5393: f64, t44601: f64, t10703: f64, t10758: f64, t1212: f64, t1248: f64, t1255: f64, t15128: f64, t1901: f64, t21351: f64, t21369: f64, t22161: f64, t22410: f64, t2862: f64, t296: f64, t319: f64, t44445: f64, t446: f64, t4973: f64, t5330: f64, t835: f64, t840: f64, t84628: f64, t84630: f64, t871: f64, t88726: f64, t88730: f64) -> (f64, f64, f64) {
    let t90873 = t10697 * t5309 * t5393;
    let t90935 = t5309 * t5309;
    let t90936 = t44601 * t90935;
    let t90940 = -8.0_f64 * t446 * t840 * t15128 * t22410 + 8.0_f64 / 3.0_f64 * t446 * t2862 * t319 * t22161 * t1212 + 4.0_f64 / 3.0_f64 * t446 * t840 * t871 * t22161 * t1248 - 4.0_f64 / 9.0_f64 * t446 * t835 * t1255 * t21369 - t446 * t835 * t319 * t88730 / 9.0_f64 - 40.0_f64 / 81.0_f64 * t446 * t10758 * t1255 * t21351 - 80.0_f64 / 243.0_f64 * t446 * t44445 * t319 * t88726 + 4.0_f64 / 27.0_f64 * t84628 + 40.0_f64 / 243.0_f64 * t84630 - 4.0_f64 / 3.0_f64 * t1901 * t10703 * t5330 * t4973 + 8.0_f64 * t446 * t296 * t90936;
    (t90873, t90936, t90940)
}
