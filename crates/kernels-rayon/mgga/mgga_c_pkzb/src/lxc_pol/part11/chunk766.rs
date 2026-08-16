//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 766/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk766(t410: f64, t6523: f64, t2370: f64, t6012: f64, t2393: f64, t937: f64, t6455: f64, t394: f64, t448: f64, t452: f64, t1424: f64, t7: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6569 = t6523 * t410;
    let t6570 = t6012 * t2370;
    let t6579 = t2393 * t937;
    let t6590 = t6455 * t410;
    let t6591 = t6012 * t394;
    let t6634 = t448 * t452;
    let t6658 = t7 * t1424;
    (t6569, t6570, t6579, t6590, t6591, t6634, t6658)
}
