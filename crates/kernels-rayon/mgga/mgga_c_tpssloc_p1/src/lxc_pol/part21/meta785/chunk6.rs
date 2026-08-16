//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2725/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2725(t12300: f64, t6417: f64, t19868: f64, t3799: f64, t12283: f64, t19958: f64, t12351: f64, t12407: f64, t12429: f64, t1363: f64, t16018: f64, t16060: f64, t16148: f64, t16153: f64, t16224: f64, t16391: f64, t1799: f64, t1825: f64, t19876: f64, t19882: f64, t19956: f64, t3719: f64, t3803: f64, t3805: f64, t3807: f64, t3870: f64, t40293: f64, t5245: f64, t5252: f64, t54585: f64, t54607: f64, t54609: f64, t54611: f64, t54750: f64, t56817: f64, t6330: f64, t820: f64) -> f64 {
    let t57407 = t12300 * t6417;
    let t57409 = t3799 * t19868;
    let t57437 = t12283 * t19958;
    let t57447 = -5.0_f64 / 128.0_f64 * t1363 * t12351 * t820 * t6330 * t3719 + 7.0_f64 / 2304.0_f64 * t57407 + 7.0_f64 / 2304.0_f64 * t57409 + 7.0_f64 / 2304.0_f64 * t54585 + 5.0_f64 / 384.0_f64 * t1363 * t3870 * t820 * t1799 * t16018 - 119.0_f64 / 1728.0_f64 * t40293 - 7.0_f64 / 576.0_f64 * t54607 + t3803 * t3805 * t19956 * t12407 / 768.0_f64 - 7.0_f64 / 2304.0_f64 * t54609 - t19876 * t16391 / 192.0_f64 + t16060 * t5245 * t5252 / 384.0_f64 + 119.0_f64 / 864.0_f64 * t54611 + 7.0_f64 / 288.0_f64 * t54750 + t3803 * t3805 * t56817 * t3807 / 384.0_f64 + t12429 * t19882 / 384.0_f64 - 7.0_f64 / 576.0_f64 * t57437 - 5.0_f64 / 192.0_f64 * t3803 * t16224 * t1825 * t16148 - 5.0_f64 / 384.0_f64 * t3803 * t16224 * t1825 * t16153;
    t57447
}
