//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1203/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1203(t35709: f64, t31501: f64, t31503: f64, t31505: f64, t31510: f64, t31514: f64, t31526: f64, t31528: f64, t31530: f64, t31532: f64, t32833: f64, t32834: f64, t32839: f64, t32844: f64, t32850: f64, t35720: f64, t35722: f64, t35724: f64) -> f64 {
    let t37675 = 0.64025200389650807212e-1_f64 * t35709;
    let t37688 = -t32833 - t32834 - t37675 + 0.64311027177104605458e-2_f64 * t31501 - 0.77173232612525526552e-2_f64 * t31503 - 0.36014175219178579058e-1_f64 * t31505 - t32839 - 7.0_f64 / 72.0_f64 * t31510 - 11.0_f64 / 288.0_f64 * t31514 + t32844 + 0.79249192569802463213e-1_f64 * t31526 + 0.22642626448514989489e-1_f64 * t31528 + 0.68598428988911579156e-2_f64 * t31530 - 0.68598428988911579156e-2_f64 * t31532 + 0.34299214494455789578e-1_f64 * t35720 + t32850 + 0.51448821741683684366e-2_f64 * t35722 + 0.13719685797782315831e-1_f64 * t35724;
    t37688
}
