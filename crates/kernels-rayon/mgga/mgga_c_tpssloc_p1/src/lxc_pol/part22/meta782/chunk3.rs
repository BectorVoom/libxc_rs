//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2675/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2675(t12286: f64, t12351: f64, t1307: f64, t1341: f64, t1343: f64, t1363: f64, t1799: f64, t19631: f64, t19921: f64, t19926: f64, t20416: f64, t20497: f64, t20556: f64, t20565: f64, t3778: f64, t3783: f64, t3870: f64, t5187: f64, t5240: f64, t56776: f64, t56779: f64, t56795: f64, t56797: f64, t6330: f64, t6347: f64, t74564: f64, t820: f64) -> f64 {
    let t74569 = 7.0_f64 / 192.0_f64 * t56776 + 7.0_f64 / 192.0_f64 * t56779 - 119.0_f64 / 576.0_f64 * t56795 + 7.0_f64 / 384.0_f64 * t56797 + t12286 * t20497 / 512.0_f64 + 5.0_f64 / 768.0_f64 * t1363 * t3870 * t820 * t20416 * t1307 - 15.0_f64 / 128.0_f64 * t1363 * t12351 * t820 * t6330 * t5187 + 5.0_f64 / 256.0_f64 * t3783 * t20565 + 5.0_f64 / 256.0_f64 * t1363 * t3870 * t820 * t5187 * t6347 + 5.0_f64 / 256.0_f64 * t1363 * t3870 * t820 * t1799 * t19631 - 15.0_f64 / 128.0_f64 * t5240 * t19921 + 5.0_f64 / 128.0_f64 * t5240 * t19926 - t3778 * t20556 / 3072.0_f64 - t1341 * t1343 * t820 * t74564 / 3072.0_f64;
    t74569
}
