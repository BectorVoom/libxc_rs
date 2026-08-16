//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1201/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1201<F: Float>(t10572: F, t5250: F, t6897: F, t8909: F, t10659: F, t17043: F, t1034: F, t164: F, t8888: F, t2639: F, t3441: F, t17067: F, t179: F, t20222: F, t20242: F, t20262: F, t20275: F, t20407: F, t20409: F, t24402: F, t24421: F, t2592: F, t2645: F, t2653: F, t29012: F, t5244: F, t568: F, t6896: F, t8914: F, t8953: F) -> (F, F, F, F, F) {
    let t29399 = t10572 * t5250;
    let t29403 = t6897 * t8909;
    let t29407 = t17043 * t10659;
    let t29410 = t8888 * t1034 * t164;
    let t29415 = t3441 * t2639 * t164;
    let t29423 = -t20222 + F::cast_from(0.68026775414003982663e-1_f64) * t20242 - t20262 - t20275 + F::cast_from(0.34013387707001991332e0_f64) * t20407 + F::cast_from(455.0_f64) / F::cast_from(216.0_f64) * t20409 - F::cast_from(0.12004725073059526352e-1_f64) * t24402 - F::cast_from(0.18007087609589289528e-1_f64) * t24421 + F::cast_from(0.1543464652250510531e-1_f64) * t17067 * t179 * t8953 * t2653 - F::cast_from(0.1543464652250510531e-1_f64) * t5244 * t179 * t8914 * t2653 + F::cast_from(0.38586616306262763276e-2_f64) * t2592 * t179 * t29399 - F::cast_from(0.38586616306262763276e-2_f64) * t6896 * t179 * t29403 + F::cast_from(0.60023625365297631763e-1_f64) * t29407 - F::cast_from(0.64311027177104605458e-3_f64) * t2645 * t179 * t29410 - F::cast_from(0.64311027177104605458e-3_f64) * t2645 * t179 * t29415 + F::cast_from(0.51448821741683684368e-2_f64) * t17067 * t179 * t29012 * t568;
    (t29399, t29403, t29410, t29415, t29423)
}
