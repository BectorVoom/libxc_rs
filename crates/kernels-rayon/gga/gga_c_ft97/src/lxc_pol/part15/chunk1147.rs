//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1147/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1147(t1168: f64, t21639: f64, t2568: f64, t10024: f64, t1175: f64, t18391: f64, t21351: f64, t2594: f64, t265: f64, t42469: f64, t446: f64, t4973: f64, t5053: f64, t5064: f64, t5073: f64, t5181: f64, t68200: f64, t724: f64, t729: f64, t88726: f64, t88730: f64, t88742: f64, t88749: f64, t88756: f64, t88764: f64) -> (f64, f64) {
    let t89273 = t2568 * t1168 * t21639;
    let t89343 = -40.0_f64 / 81.0_f64 * t446 * t10024 * t1175 * t21351 - 80.0_f64 / 243.0_f64 * t446 * t42469 * t265 * t88726 - t446 * t724 * t265 * t88730 / 9.0_f64 + 8.0_f64 / 9.0_f64 * t68200 - 2.0_f64 / 3.0_f64 * t446 * t724 * t5181 * t4973 - 4.0_f64 * t446 * t729 * t2568 * t5053 * t5064 + 8.0_f64 / 3.0_f64 * t446 * t724 * t265 * t88749 + 4.0_f64 * t446 * t729 * t18391 * t5073 + 40.0_f64 / 27.0_f64 * t446 * t10024 * t265 * t88764 - 2.0_f64 / 9.0_f64 * t446 * t2594 * t265 * t88742 + 2.0_f64 / 3.0_f64 * t446 * t724 * t265 * t88756;
    (t89273, t89343)
}
