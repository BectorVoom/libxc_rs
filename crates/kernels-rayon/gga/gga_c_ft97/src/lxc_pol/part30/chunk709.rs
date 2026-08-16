//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 709/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk709(t1248: f64, t6260: f64, t840: f64, t871: f64, t1476: f64, t4299: f64, t1255: f64, t2862: f64, t6278: f64, t4246: f64, t6287: f64, t7036: f64, t882: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t29302 = t6260 * t1248;
    let t29304 = t840 * t871 * t29302;
    let t29307 = t1476 * t4299;
    let t29309 = t840 * t871 * t29307;
    let t29313 = t2862 * t1255 * t6278;
    let t29317 = t840 * t4246 * t6287;
    let t29321 = t2862 * t882 * t7036;
    (t29302, t29304, t29307, t29309, t29313, t29317, t29321)
}
