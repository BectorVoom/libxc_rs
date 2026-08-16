//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1435/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1435(t1873: f64, t3941: f64, t9416: f64, t16535: f64, t6534: f64, t45557: f64, t45560: f64, t7015: f64, t20173: f64, t23896: f64, t112: f64, t23862: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t83991 = 27.0_f64 * t3941 * t1873 * t9416;
    let t83993 = 81.0_f64 * t16535 * t6534;
    let t83999 = 0.135e2_f64 * t45557 * t1873;
    let t84001 = 81.0_f64 * t45560 * t7015;
    let t84003 = 81.0_f64 * t20173 * t23896;
    let t84004 = t23862 * t112;
    (t83991, t83993, t83999, t84001, t84003, t84004)
}
