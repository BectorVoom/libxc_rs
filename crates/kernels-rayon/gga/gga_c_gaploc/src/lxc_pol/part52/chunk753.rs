//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 753/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk753(t1339: f64, t31585: f64, t2754: f64, t4130: f64, t10241: f64, t20550: f64, t2875: f64, t544: f64, t6514: f64, t1359: f64, t2925: f64, t299: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t35045 = t1339 * t31585;
    let t35091 = t4130 * t2754;
    let t35101 = t20550 * t10241;
    let t35180 = t544 * t6514 * t2875;
    let t35215 = t1359 * t10241;
    let t35216 = t544 * t35215;
    let t35385 = t299 * t2925;
    (t35045, t35091, t35101, t35180, t35215, t35216, t35385)
}
