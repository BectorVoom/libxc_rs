//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1214/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1214(t1041: f64, t10489: f64, t3103: f64, t3109: f64, t3114: f64, t376: f64, t676: f64, t1023: f64, t248: f64, t1020: f64, t1017: f64, t3087: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10490 = t1041 * t10489;
    let t10496 = t3109 * t3103;
    let t10504 = t3114 * t3103;
    let t10508 = t676 * t376;
    let t10510 = t248 * t10508 * t1023;
    let t10511 = t1020 * t10510;
    let t10515 = t3087 * t1017;
    (t10490, t10496, t10504, t10508, t10511, t10515)
}
