//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2705/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2705(t12339: f64, t6427: f64, t6431: f64, t12345: f64, t19815: f64, t3865: f64, t1369: f64, t1362: f64, t56923: f64, t1363: f64, t19904: f64, t3870: f64, t3872: f64, t3876: f64, t40006: f64, t40008: f64, t40019: f64, t40060: f64, t54213: f64, t54220: f64, t54222: f64, t54237: f64, t56486: f64, t820: f64) -> f64 {
    let t57007 = t12339 * t6427;
    let t57009 = t12339 * t6431;
    let t57011 = t12345 * t6427;
    let t57019 = t12345 * t6431;
    let t57021 = t19815 * t3865;
    let t57022 = t57021 * t1369;
    let t57024 = t56923 * t1362;
    let t57030 = 455.0_f64 / 324.0_f64 * t40006 - 35.0_f64 / 216.0_f64 * t40008 + 35.0_f64 / 72.0_f64 * t40019 - 7.0_f64 / 384.0_f64 * t54213 - 7.0_f64 / 288.0_f64 * t54220 - 7.0_f64 / 288.0_f64 * t54222 - 7.0_f64 / 288.0_f64 * t54237 - 35.0_f64 / 576.0_f64 * t57007 + 7.0_f64 / 576.0_f64 * t57009 + 595.0_f64 / 3456.0_f64 * t57011 + 5.0_f64 / 768.0_f64 * t19904 * t3872 + 5.0_f64 / 384.0_f64 * t1363 * t3870 * t820 * t56486 - 119.0_f64 / 3456.0_f64 * t57019 + 7.0_f64 / 576.0_f64 * t57022 - t57024 * t1369 / 384.0_f64 - t19904 * t3876 / 768.0_f64 + 595.0_f64 / 1296.0_f64 * t40060;
    t57030
}
