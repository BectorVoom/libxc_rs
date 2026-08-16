//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2339/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2339(t27817: f64, t7999: f64, t1238: f64, t14972: f64, t15797: f64, t1716: f64, t1751: f64, t17635: f64, t19209: f64, t19220: f64, t19232: f64, t24589: f64, t24601: f64, t27444: f64, t27453: f64, t27766: f64, t29795: f64, t3593: f64, t3598: f64, t460: f64, t4940: f64, t498: f64, t6267: f64, t7283: f64, t7286: f64, t7351: f64, t7391: f64, t7392: f64, t8054: f64, t8061: f64, t86473: f64, t95834: f64) -> f64 {
    let t104589 = t7999 * t27817;
    let t104596 = 2.0_f64 * t1238 * t3598 * t7391 * t6267 - 0.54831135561607547884e-2_f64 * t24589 * t24601 * t27444 * t17635 - 0.16449340668482264365e-1_f64 * t7283 * t27453 * t460 * t1751 * t7286 - t95834 - t19232 * t7392 + 2.0_f64 * t4940 * t8054 * t498 - 0.16449340668482264365e-1_f64 * t7283 * t1716 * t27766 + 2.0_f64 * t7351 * t19220 + 4.0_f64 * t14972 * t8061 - 0.14621636149762012769e-1_f64 * t104589 - t3593 * t29795 + 4.0_f64 * t15797 * t8061 + 0.6092348395734171987e-3_f64 * t86473 - t7351 * t19209;
    t104596
}
