//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1201/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1201(t10572: f64, t5250: f64, t6897: f64, t8909: f64, t10659: f64, t17043: f64, t1034: f64, t164: f64, t8888: f64, t2639: f64, t3441: f64, t17067: f64, t179: f64, t20222: f64, t20242: f64, t20262: f64, t20275: f64, t20407: f64, t20409: f64, t24402: f64, t24421: f64, t2592: f64, t2645: f64, t2653: f64, t29012: f64, t5244: f64, t568: f64, t6896: f64, t8914: f64, t8953: f64) -> (f64, f64, f64, f64, f64) {
    let t29399 = t10572 * t5250;
    let t29403 = t6897 * t8909;
    let t29407 = t17043 * t10659;
    let t29410 = t8888 * t1034 * t164;
    let t29415 = t3441 * t2639 * t164;
    let t29423 = -t20222 + 0.68026775414003982663e-1_f64 * t20242 - t20262 - t20275 + 0.34013387707001991332e0_f64 * t20407 + 455.0_f64 / 216.0_f64 * t20409 - 0.12004725073059526352e-1_f64 * t24402 - 0.18007087609589289528e-1_f64 * t24421 + 0.1543464652250510531e-1_f64 * t17067 * t179 * t8953 * t2653 - 0.1543464652250510531e-1_f64 * t5244 * t179 * t8914 * t2653 + 0.38586616306262763276e-2_f64 * t2592 * t179 * t29399 - 0.38586616306262763276e-2_f64 * t6896 * t179 * t29403 + 0.60023625365297631763e-1_f64 * t29407 - 0.64311027177104605458e-3_f64 * t2645 * t179 * t29410 - 0.64311027177104605458e-3_f64 * t2645 * t179 * t29415 + 0.51448821741683684368e-2_f64 * t17067 * t179 * t29012 * t568;
    (t29399, t29403, t29410, t29415, t29423)
}
