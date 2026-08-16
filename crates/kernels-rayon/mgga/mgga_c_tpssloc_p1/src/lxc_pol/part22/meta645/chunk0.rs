//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2185/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2185(t12283: f64, t19991: f64, t40281: f64, t6396: f64, t12339: f64, t6427: f64, t6431: f64, t12345: f64, t19815: f64, t3865: f64, t1369: f64, t1362: f64, t56923: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t56963 = t12283 * t19991;
    let t56993 = t40281 * t6396;
    let t57007 = t12339 * t6427;
    let t57009 = t12339 * t6431;
    let t57011 = t12345 * t6427;
    let t57019 = t12345 * t6431;
    let t57021 = t19815 * t3865;
    let t57022 = t57021 * t1369;
    let t57024 = t56923 * t1362;
    (t56963, t56993, t57007, t57009, t57011, t57019, t57021, t57022, t57024)
}
