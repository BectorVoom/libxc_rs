//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 625/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk625(t5676: f64, t6028: f64, t6027: f64, t1529: f64, t2047: f64, t1547: f64, t2061: f64, t1546: f64, t556: f64, t5627: f64, t572: f64, t1533: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6029 = t6028 * t5676;
    let t6030 = t6027 * t6029;
    let t6032 = t1529 * t2047;
    let t6034 = t2061 * t1547;
    let t6035 = t1546 * t6034;
    let t6037 = t556 * t5627;
    let t6038 = t572 * t6037;
    let t6039 = t1533 * t6038;
    (t6029, t6030, t6032, t6034, t6035, t6037, t6038, t6039)
}
