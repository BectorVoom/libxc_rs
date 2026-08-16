//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 998/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk998(t1527: f64, t419: f64, t85456: f64, t37821: f64, t58708: f64, t58719: f64, t74068: f64, t74126: f64, t74143: f64, t74148: f64, t74153: f64, t74162: f64, t85454: f64) -> (f64, f64) {
    let t85458 = t419 * t1527 * t85456;
    let t85460 = 0.85124811172839506172e-2_f64 * t74162 - t37821 - 0.85124811172839506172e-2_f64 * t58708 - 0.51074886703703703704e-1_f64 * t74126 + 0.34049924469135802468e-1_f64 * t74068 + 0.51074886703703703704e-1_f64 * t74143 + 0.26483274587105624143e-1_f64 * t74148 - 0.68099848938271604939e-1_f64 * t74153 - 0.1134997482304526749e-1_f64 * t58719 - 0.38306165027777777778e-1_f64 * t85454 - 0.51074886703703703704e-1_f64 * t85458;
    (t85458, t85460)
}
