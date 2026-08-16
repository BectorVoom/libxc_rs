//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1163/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1163(t2349: f64, t3849: f64, t1220: f64, t8339: f64, t154: f64, t2347: f64, t385: f64, t9795: f64, t8329: f64, t10189: f64, t410: f64, t2393: f64) -> (f64, f64, f64, f64, f64) {
    let t28374 = t3849 * t2349;
    let t28376 = t1220 * t8339;
    let t28380 = t385 * t154 * t2347 * t9795;
    let t28384 = t1220 * t8329;
    let t28456 = t410 * t10189;
    let t28457 = t2393 * t28456;
    (t28374, t28376, t28380, t28384, t28457)
}
