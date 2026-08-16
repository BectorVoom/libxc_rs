//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1158/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1158(t1238: f64, t8245: f64, t179: f64, t3730: f64, t404: f64, t6380: f64, t8397: f64, t2395: f64, t3876: f64, t5939: f64, t6404: f64, t5728: f64, t919: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t28113 = t1238 * t8245;
    let t28121 = t404 * t179 * t6380 * t3730;
    let t28123 = t1238 * t8397;
    let t28128 = t2395 * t5939 * t3876;
    let t28138 = t6404 * t3730;
    let t28147 = t5728 * t919;
    (t28113, t28121, t28123, t28128, t28138, t28147)
}
