//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1282/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1282(t12210: f64, t40681: f64, t37580: f64, t38211: f64, t39106: f64, t40556: f64, t40559: f64, t40564: f64, t40587: f64, t43921: f64, t44940: f64, t44942: f64, t45023: f64, t45026: f64, t45030: f64, t45034: f64) -> (f64, f64) {
    let t45036 = 3.0_f64 / 2.0_f64 * t40681 * t12210;
    let t45040 = -t44940 + t44942 + 0.325201597776800302e-2_f64 * t40556 + 0.38422568777328955681e-2_f64 * t40559 - 0.17347588262831798123e-3_f64 * t40564 + t45023 + t45026 - t45030 + 0.68400385060046895e-6_f64 * t37580 - t45034 - t45036 + 0.3842256877732895568e-2_f64 * t43921 - 0.32326021979378162576e-5_f64 * t40587 + 0.60975299583150056624e-3_f64 * t38211 - t39106;
    (t45036, t45040)
}
