//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 694/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk694(t1745: f64, t732: f64, t1731: f64, t5311: f64, t5314: f64, t636: f64, t12: f64, t3: f64, t40: f64, t1737: f64, t4735: f64, t4738: f64, t640: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5413 = t732 * t1745;
    let t5416 = t1731 * t5311;
    let t5418 = t636 * t5314;
    let t5420 = f64::powf(t12, -0.25e1_f64);
    let t5421 = t5420 * t3;
    let t5422 = t5421 * t40;
    let t5424 = t1737 * t4735;
    let t5426 = t640 * t4738;
    (t5413, t5416, t5418, t5422, t5424, t5426)
}
