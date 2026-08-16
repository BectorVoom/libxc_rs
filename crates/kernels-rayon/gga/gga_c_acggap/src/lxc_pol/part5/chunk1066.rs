//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1066/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1066(t3073: f64, t4166: f64, t4241: f64, t4146: f64, t1629: f64, t17056: f64, t12349: f64, t12730: f64, t17775: f64, t13079: f64, t4184: f64, t3378: f64, t4153: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t18880 = t3073 * t4166 * t4241;
    let t18884 = t3073 * t4146 * t4241;
    let t18887 = t3073 * t1629 * t17056;
    let t18890 = t3073 * t1629 * t12349;
    let t18893 = t12730 * t1629 * t17775;
    let t18895 = t13079 * t4184;
    let t18897 = t3378 * t4153;
    (t18880, t18884, t18887, t18890, t18893, t18895, t18897)
}
