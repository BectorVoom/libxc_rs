//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 935/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk935<F: Float>(t1363: F, t16211: F, t1831: F, t19834: F, t19839: F, t19841: F, t19851: F, t19904: F, t20433: F, t20442: F, t20484: F, t20508: F, t20599: F, t3803: F, t5240: F, t6427: F, t6431: F) -> F {
    let t20601 = -F::cast_from(119.0_f64) / F::cast_from(4608.0_f64) * t16211 - t5240 * t6431 / F::cast_from(256.0_f64) + F::cast_from(5.0_f64) / F::cast_from(256.0_f64) * t5240 * t6427 - F::cast_from(5.0_f64) / F::cast_from(128.0_f64) * t1363 * t20433 - t19904 * t1831 / F::cast_from(256.0_f64) - F::cast_from(7.0_f64) / F::cast_from(1536.0_f64) * t19834 - F::cast_from(7.0_f64) / F::cast_from(16.0_f64) * t19839 + F::cast_from(7.0_f64) / F::cast_from(48.0_f64) * t19841 - t3803 * t20442 / F::cast_from(1024.0_f64) - F::cast_from(7.0_f64) / F::cast_from(768.0_f64) * t19851 + t20484 + t20508 + t20599;
    t20601
}
