//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1230/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1230(t2189: f64, t71840: f64, t1020: f64, t19781: f64, t26760: f64, t20671: f64, t5329: f64, t7773: f64, t283: f64, t6708: f64, t7719: f64, t1267: f64, t28110: f64, t5310: f64, t6276: f64) -> (f64, f64, f64, f64, f64) {
    let t100034 = t71840 * t2189;
    let t100051 = t1020 * t26760 * t19781;
    let t100056 = t5329 * t7773 * t20671;
    let t100059 = t6708 * t283;
    let t100061 = t1020 * t100059 * t7719;
    let t100067 = t5310 * t28110 * t6276 * t1267;
    (t100034, t100051, t100056, t100061, t100067)
}
