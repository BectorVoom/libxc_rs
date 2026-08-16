//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2037/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2037(t100988: f64, t101012: f64, t101055: f64, t101083: f64, t101099: f64, t103570: f64, t1940: f64, t2403: f64, t25781: f64, t25784: f64, t26425: f64, t26581: f64, t26585: f64, t27770: f64, t27777: f64, t27802: f64, t27810: f64, t28456: f64, t28460: f64, t7200: f64, t7428: f64, t7432: f64, t7862: f64, t7869: f64, t95511: f64, t95527: f64) -> f64 {
    let t103853 = 3.0_f64 * t2403 * t28456 * t7200 - t103570 + 3.0_f64 * t2403 * t7428 * t27777 - t1940 * t7432 * t101012 / 2.0_f64 - 3.0_f64 * t26425 * t101055 + 3.0_f64 * t2403 * t7428 * t27810 - t1940 * t28460 * t25784 / 2.0_f64 - t1940 * t26585 * t27802 - 3.0_f64 * t26425 * t101083 - t1940 * t28460 * t25781 + 3.0_f64 / 2.0_f64 * t2403 * t26581 * t7862 - 3.0_f64 * t95511 * t27770 - t1940 * t95527 * t7869 / 2.0_f64 - 3.0_f64 * t26425 * t100988 - t1940 * t7432 * t101099;
    t103853
}
