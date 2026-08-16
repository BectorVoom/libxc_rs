//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 932/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk932(t10007: f64, t18412: f64, t14163: f64, t17785: f64, t5053: f64, t729: f64, t773: f64, t18139: f64, t265: f64, t1882: f64, t5176: f64, t13872: f64, t13875: f64, t13884: f64, t13903: f64, t13905: f64, t13933: f64, t18399: f64, t18403: f64, t18406: f64, t18409: f64, t1901: f64, t446: f64) -> f64 {
    let t18413 = t10007 * t18412;
    let t18416 = t14163 * t17785;
    let t18420 = t729 * t773 * t5053;
    let t18424 = t729 * t265 * t18139;
    let t18427 = t1882 * t5176;
    let t18429 = -8.0_f64 / 27.0_f64 * t13872 + t13875 - t446 * t18399 / 3.0_f64 - t13884 - 2.0_f64 / 9.0_f64 * t1901 * t18403 - 4.0_f64 / 9.0_f64 * t1901 * t18406 + 4.0_f64 / 27.0_f64 * t1901 * t18409 - 2.0_f64 / 9.0_f64 * t1901 * t18413 - 4.0_f64 / 9.0_f64 * t1901 * t18416 - t446 * t18420 / 3.0_f64 - t446 * t18424 / 3.0_f64 + t13903 + t13905 - 2.0_f64 / 27.0_f64 * t18427 + t13933;
    t18429
}
