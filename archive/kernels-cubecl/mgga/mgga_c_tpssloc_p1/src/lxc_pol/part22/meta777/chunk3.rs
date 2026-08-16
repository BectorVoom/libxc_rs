//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2658/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2658<F: Float>(t20595: F, t68: F, t1340: F, t20556: F, t3799: F, t20570: F, t1362: F, t1354: F, t1369: F, t16278: F, t16321: F, t16394: F, t1831: F, t19868: F, t19904: F, t19930: F, t19991: F, t20479: F, t20492: F, t3783: F, t39936: F, t40035: F, t5235: F, t5240: F, t5314: F, t57024: F, t6417: F, t6431: F) -> (F, F) {
    let t74289 = t20595 * t68;
    let t74290 = t74289 * t1340;
    let t74297 = t3799 * t20556;
    let t74299 = t3799 * t20570;
    let t74311 = t74289 * t1362;
    let t74316 = t16394 * t19991 / F::cast_from(128.0_f64) + t39936 - t74290 * t1354 / F::cast_from(3072.0_f64) - t16278 * t6417 / F::cast_from(1024.0_f64) - t5235 * t19868 / F::cast_from(1024.0_f64) + F::cast_from(7.0_f64) / F::cast_from(4608.0_f64) * t74297 + F::cast_from(7.0_f64) / F::cast_from(4608.0_f64) * t74299 - t40035 * t20492 / F::cast_from(512.0_f64) - t16321 * t6431 / F::cast_from(256.0_f64) - t5240 * t19930 / F::cast_from(256.0_f64) - t57024 * t1831 / F::cast_from(256.0_f64) - t19904 * t5314 / F::cast_from(256.0_f64) - t74311 * t1369 / F::cast_from(768.0_f64) - t3783 * t20479 / F::cast_from(768.0_f64);
    (t74289, t74316)
}
