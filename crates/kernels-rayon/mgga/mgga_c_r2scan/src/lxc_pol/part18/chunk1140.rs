//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1140/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1140(t3579: f64, t40590: f64, t10610: f64, t11479: f64, t11509: f64, t12574: f64, t792: f64, t3275: f64, t37299: f64, t12602: f64, t833: f64, t23495: f64, t3629: f64) -> (f64, f64, f64, f64, f64) {
    let t42467 = 5.0_f64 / 8.0_f64 * t3579 * t40590;
    let t42471 = 3.0_f64 * t10610 * t11479 * t11509;
    let t42472 = t12574 * t792;
    let t42475 = 585.0_f64 / 256.0_f64 * t3275 * t37299 * t42472;
    let t42478 = t12602 * t833;
    let t42491 = t23495 * t3629;
    (t42467, t42471, t42475, t42478, t42491)
}
