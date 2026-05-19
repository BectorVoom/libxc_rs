//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1282/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1282<F: Float>(t12210: F, t40681: F, t37580: F, t38211: F, t39106: F, t40556: F, t40559: F, t40564: F, t40587: F, t43921: F, t44940: F, t44942: F, t45023: F, t45026: F, t45030: F, t45034: F) -> (F, F) {
    let t45036 = F::new(3.0) / F::new(2.0) * t40681 * t12210;
    let t45040 = -t44940 + t44942 + F::cast_from(0.325201597776800302e-2_f64) * t40556 + F::cast_from(0.38422568777328955681e-2_f64) * t40559 - F::cast_from(0.17347588262831798123e-3_f64) * t40564 + t45023 + t45026 - t45030 + F::cast_from(0.68400385060046895e-6_f64) * t37580 - t45034 - t45036 + F::cast_from(0.3842256877732895568e-2_f64) * t43921 - F::cast_from(0.32326021979378162576e-5_f64) * t40587 + F::cast_from(0.60975299583150056624e-3_f64) * t38211 - t39106;
    (t45036, t45040)
}
