//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2658/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2658(t20595: f64, t68: f64, t1340: f64, t20556: f64, t3799: f64, t20570: f64, t1362: f64, t1354: f64, t1369: f64, t16278: f64, t16321: f64, t16394: f64, t1831: f64, t19868: f64, t19904: f64, t19930: f64, t19991: f64, t20479: f64, t20492: f64, t3783: f64, t39936: f64, t40035: f64, t5235: f64, t5240: f64, t5314: f64, t57024: f64, t6417: f64, t6431: f64) -> (f64, f64) {
    let t74289 = t20595 * t68;
    let t74290 = t74289 * t1340;
    let t74297 = t3799 * t20556;
    let t74299 = t3799 * t20570;
    let t74311 = t74289 * t1362;
    let t74316 = t16394 * t19991 / 128.0_f64 + t39936 - t74290 * t1354 / 3072.0_f64 - t16278 * t6417 / 1024.0_f64 - t5235 * t19868 / 1024.0_f64 + 7.0_f64 / 4608.0_f64 * t74297 + 7.0_f64 / 4608.0_f64 * t74299 - t40035 * t20492 / 512.0_f64 - t16321 * t6431 / 256.0_f64 - t5240 * t19930 / 256.0_f64 - t57024 * t1831 / 256.0_f64 - t19904 * t5314 / 256.0_f64 - t74311 * t1369 / 768.0_f64 - t3783 * t20479 / 768.0_f64;
    (t74289, t74316)
}
