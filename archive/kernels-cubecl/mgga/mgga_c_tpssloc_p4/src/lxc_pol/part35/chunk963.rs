//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 963/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk963<F: Float>(t20594: F, t225: F, t554: F, t12215: F, t1341: F, t1363: F, t16285: F, t1827: F, t19855: F, t19940: F, t19942: F, t20512: F, t20516: F, t20556: F, t20565: F, t20570: F, t3733: F, t5235: F, t559: F, t6390: F, t6422: F) -> (F, F) {
    let t20595 = t20594 * t225;
    let t20596 = t20595 * t554;
    let t20599 = -F::cast_from(35.0_f64) / F::cast_from(384.0_f64) * t19940 + F::cast_from(7.0_f64) / F::cast_from(384.0_f64) * t19942 - t12215 * t20512 / F::cast_from(4.0_f64) + F::cast_from(3.0_f64) / F::cast_from(16.0_f64) * t3733 * t20516 - t1341 * t20556 / F::cast_from(3072.0_f64) - t5235 * t6422 / F::cast_from(1024.0_f64) + t16285 * t6390 / F::cast_from(512.0_f64) + F::cast_from(5.0_f64) / F::cast_from(256.0_f64) * t1363 * t20565 - t1341 * t20570 / F::cast_from(3072.0_f64) - t19855 * t1827 / F::cast_from(1024.0_f64) + t20596 * t559 / F::cast_from(3072.0_f64);
    (t20595, t20599)
}
