//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1137/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1137(t4479: f64, t6220: f64, t1505: f64, t17306: f64, t1628: f64, t18266: f64, t1610: f64, t6183: f64, t2104: f64, t4463: f64, t110: f64, t494: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t52930 = t6220 * t4479;
    let t52933 = t17306 * t1505;
    let t52955 = t18266 * t1628;
    let t53436 = t6183 * t1610;
    let t53551 = t2104 * t4463;
    let t54162 = t110 * t494;
    (t52930, t52933, t52955, t53436, t53551, t54162)
}
