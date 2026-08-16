//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 812/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk812(t1429: f64, t8624: f64, t1424: f64, t3318: f64, t440: f64, t4803: f64, t15: f64, t3329: f64, t4810: f64, t444: f64, t8: f64, t983: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8625 = t8624 * t1429;
    let t8630 = t1424 * t3318;
    let t8631 = t8630 * t440;
    let t8635 = -t1429 - 3.0_f64 * t4803;
    let t8636 = t15 * t8635;
    let t8645 = t4810 * t3329;
    let t8646 = t8645 * t444;
    let t8649 = t983 * t8;
    (t8625, t8631, t8635, t8636, t8646, t8649)
}
