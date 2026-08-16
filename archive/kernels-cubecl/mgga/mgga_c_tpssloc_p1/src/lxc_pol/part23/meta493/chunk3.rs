//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1515/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1515<F: Float>(t12419: F, t19871: F, t19956: F, t20448: F, t20463: F, t20468: F, t3803: F, t3805: F, t39936: F, t5246: F, t74120: F, t74258: F, t74260: F, t74274: F, t74276: F, t74297: F, t74299: F, t74360: F, t74376: F, t74393: F) -> F {
    let t80352 = F::cast_from(7.0_f64) / F::cast_from(96.0_f64) * t74258 + F::cast_from(7.0_f64) / F::cast_from(96.0_f64) * t74260 - t5246 * t3805 * t74120 * t20468 / F::cast_from(32.0_f64) - F::cast_from(7.0_f64) / F::cast_from(48.0_f64) * t74274 + F::cast_from(35.0_f64) / F::cast_from(96.0_f64) * t74276 + t39936 + F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t74297 + F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t74299 + F::cast_from(7.0_f64) / F::cast_from(3.0_f64) * t74360 + F::cast_from(7.0_f64) / F::cast_from(384.0_f64) * t74376 - F::cast_from(5.0_f64) / F::cast_from(128.0_f64) * t3803 * t12419 * t19956 * t20448 + t3803 * t3805 * t19871 * t20463 / F::cast_from(128.0_f64) - F::cast_from(7.0_f64) / F::cast_from(4.0_f64) * t74393;
    t80352
}
