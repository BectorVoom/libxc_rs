//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2723/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2723(t12300: f64, t6422: f64, t12365: f64, t1358: f64, t19836: f64, t12250: f64, t6387: f64, t12429: f64, t16101: f64, t16215: f64, t16217: f64, t16225: f64, t16233: f64, t16305: f64, t16311: f64, t16312: f64, t16401: f64, t1825: f64, t19735: f64, t19886: f64, t19890: f64, t221: f64, t3803: f64, t5240: f64, t5246: f64, t53973: f64, t54063: f64, t54555: f64, t54557: f64, t54561: f64, t54567: f64, t56560: f64, t57086: f64, t6388: f64, t6394: f64) -> f64 {
    let t57308 = t12300 * t6422;
    let t57310 = t12365 * t6422;
    let t57324 = t19836 * t1358;
    let t57342 = t6387 * t12250;
    let t57351 = 7.0_f64 / 2304.0_f64 * t57308 - 119.0_f64 / 13824.0_f64 * t57310 - 119.0_f64 / 3456.0_f64 * t54555 + 7.0_f64 / 2304.0_f64 * t54557 - 7.0_f64 / 1152.0_f64 * t54561 + 7.0_f64 / 384.0_f64 * t54567 - t16401 * t19890 / 96.0_f64 - t5246 * t16305 * t19735 * t16225 / 96.0_f64 - 5.0_f64 / 64.0_f64 * t5240 * t16217 - 7.0_f64 / 2304.0_f64 * t57324 - t5246 * t16305 * t16311 * t57086 / 96.0_f64 + t12429 * t19886 / 192.0_f64 + t3803 * t16305 * t53973 * t6394 / 192.0_f64 - t16101 * t221 * t56560 + 5.0_f64 / 64.0_f64 * t3803 * t54063 * t1825 * t16215 + t16233 * t16305 * t57342 * t16312 / 64.0_f64 - t5246 * t16305 * t6388 * t16312 / 64.0_f64;
    t57351
}
