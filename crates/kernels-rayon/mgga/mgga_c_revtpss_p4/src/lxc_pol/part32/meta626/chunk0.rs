//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1990/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1990(t102528: f64, t102530: f64, t102531: f64, t102534: f64, t102535: f64, t102537: f64, t102548: f64, t108590: f64, t108592: f64, t94498: f64, t96326: f64, t98224: f64, t98260: f64) -> f64 {
    let t109822 = -0.80031500487063509015e-2_f64 * t108590 + 0.40015750243531754507e-2_f64 * t108592 - t102528 - 0.45351183609335988441e-1_f64 * t98224 + t102530 - t102531 - t102534 + t102535 + t96326 + t102537 + 0.54208002996571016773e-3_f64 * t94498 - t102548 - 35.0_f64 / 54.0_f64 * t98260;
    t109822
}
