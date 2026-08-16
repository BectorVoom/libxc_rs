//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1174/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1174(t28924: f64, t6353: f64, t1882: f64, t36268: f64, t15133: f64, t7679: f64, t36153: f64, t8392: f64, t36149: f64, t143673: f64, t144250: f64, t144260: f64, t144271: f64, t144273: f64, t15128: f64, t15312: f64, t153555: f64, t1901: f64, t24873: f64, t29215: f64, t296: f64, t34167: f64, t4255: f64, t4260: f64, t44523: f64, t446: f64, t56352: f64, t6386: f64, t840: f64, t992: f64, t99238: f64) -> (f64, f64, f64, f64, f64) {
    let t154842 = t6353 * t28924;
    let t154849 = t1882 * t36268;
    let t154851 = t15133 * t7679;
    let t154864 = t8392 * t36153;
    let t154867 = t8392 * t36149;
    let t154896 = 4.0_f64 / 9.0_f64 * t154867 - 2.0_f64 / 9.0_f64 * t1901 * t99238 * t29215 + 2.0_f64 / 9.0_f64 * t1901 * t44523 * t143673 * t4255 + 2.0_f64 / 3.0_f64 * t1901 * t56352 * t143673 * t4260 - 4.0_f64 / 9.0_f64 * t1901 * t15312 * t24873 * t992 * t6386 - t144250 / 9.0_f64 + 2.0_f64 / 9.0_f64 * t144260 - 2.0_f64 / 3.0_f64 * t446 * t840 * t15128 * t34167 + 2.0_f64 / 3.0_f64 * t446 * t296 * t153555 + 2.0_f64 / 9.0_f64 * t144271 + 2.0_f64 / 3.0_f64 * t144273;
    (t154842, t154849, t154851, t154864, t154896)
}
