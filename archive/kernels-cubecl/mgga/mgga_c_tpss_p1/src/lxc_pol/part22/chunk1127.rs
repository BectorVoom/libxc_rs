//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1127/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1127<F: Float>(t11453: F, t4284: F, t1125: F, t12431: F, t12435: F, t12439: F, t12443: F, t12446: F, t12448: F, t12451: F, t12455: F, t12460: F, t12465: F, t12467: F, t12472: F, t12477: F, t3052: F, t3057: F, t3070: F, t3076: F, t3080: F, t3083: F, t4258: F, t9607: F, t9664: F, t9669: F, t9673: F, t9677: F, t9701: F) -> F {
    let t12478 = t11453 * t4284;
    let t12480 = t1125 * t12478 / F::cast_from(1728.0_f64);
    let t12481 = -t9664 / F::cast_from(432.0_f64) + t9669 / F::cast_from(10368.0_f64) - t9673 / F::cast_from(6912.0_f64) - t9677 / F::cast_from(3456.0_f64) - t4258 * t3076 / F::cast_from(576.0_f64) - t12431 * t3057 / F::cast_from(288.0_f64) + t12435 * t3083 / F::cast_from(576.0_f64) + t12439 + t12443 + t9701 - t12446 / F::cast_from(13824.0_f64) + t12448 / F::cast_from(2592.0_f64) - t3080 * t12451 / F::cast_from(3072.0_f64) + t9607 * t12455 / F::cast_from(3072.0_f64) - t3080 * t12460 / F::cast_from(1536.0_f64) - t12465 + t3052 * t12467 / F::cast_from(768.0_f64) + t12472 * t3070 / F::cast_from(432.0_f64) - t12477 - t12480;
    t12481
}
