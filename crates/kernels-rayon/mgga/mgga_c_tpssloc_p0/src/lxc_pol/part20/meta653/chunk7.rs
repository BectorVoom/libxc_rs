//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2415/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2415(t14363: f64, t942: f64, t10760: f64, t10806: f64, t10814: f64, t14329: f64, t14332: f64, t1569: f64, t2856: f64, t2925: f64, t42117: f64, t4411: f64, t4434: f64, t49268: f64, t49271: f64, t49273: f64, t49276: f64, t49278: f64, t49280: f64, t49282: f64, t49285: f64, t49305: f64, t49318: f64, t49332: f64, t49345: f64, t49359: f64, t49372: f64, t49386: f64, t49397: f64, t924: f64, t932: f64, t952: f64) -> f64 {
    let t49404 = t14363 * t942;
    let t49409 = -t49268 - t49271 - t49273 - t49276 - t49278 - t49280 - t49282 + 1.0_f64 * t4411 * t10806 + 0.2069040516770936012e4_f64 * t49285 * t10814 + 1.0_f64 * t42117 * t1569 + 3.0_f64 * t10760 * t4434 + 3.0_f64 * t2856 * t14329 + 1.0_f64 * t924 * (t49305 + t49318 + t49332 + t49345 + t49359 + t49372 + t49386 + t49397) * t932 + 0.17544670867903938621e1_f64 * t49404 * t952 + 0.17544670867903938621e1_f64 * t14332 * t2925;
    t49409
}
