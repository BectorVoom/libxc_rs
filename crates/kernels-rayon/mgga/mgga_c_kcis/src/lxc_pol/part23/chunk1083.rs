//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1083/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1083(t28322: f64, t446: f64, t1299: f64, t2132: f64, t2233: f64, t27364: f64, t8164: f64, t1394: f64, t167: f64, t4163: f64, t7923: f64, t5780: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t28323 = t446 * t28322;
    let t28325 = t1299 * t2132;
    let t28326 = t2233 * t28325;
    let t28328 = t27364 * t8164;
    let t28329 = t1394 * t28328;
    let t28331 = t4163 * t167;
    let t28332 = t7923 * t28331;
    let t28333 = t5780 * t28332;
    (t28323, t28326, t28328, t28329, t28331, t28332, t28333)
}
