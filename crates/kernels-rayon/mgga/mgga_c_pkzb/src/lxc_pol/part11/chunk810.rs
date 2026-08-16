//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 810/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk810(t1227: f64, t937: f64, t2363: f64, t3199: f64, t410: f64, t3258: f64, t6523: f64, t1245: f64, t3246: f64, t914: f64, t2393: f64, t3308: f64, t452: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8511 = t937 * t1227;
    let t8512 = t2363 * t8511;
    let t8515 = t410 * t3199;
    let t8516 = t2363 * t8515;
    let t8519 = t6523 * t3258;
    let t8546 = t2363 * t1245;
    let t8549 = t914 * t3246;
    let t8554 = t2393 * t1245;
    let t8599 = t3308 * t452;
    (t8512, t8516, t8519, t8546, t8549, t8554, t8599)
}
