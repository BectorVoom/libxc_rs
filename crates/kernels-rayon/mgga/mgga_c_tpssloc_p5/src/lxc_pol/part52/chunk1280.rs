//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1280/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1280(t22986: f64, t30623: f64, t86873: f64, t1903: f64, t254: f64, t23168: f64, t32789: f64, t112873: f64, t1527: f64, t1888: f64, t23270: f64, t23185: f64, t32862: f64, t82074: f64) -> (f64, f64, f64, f64, f64) {
    let t118639 = 0.3289868133696452873e-1_f64 * t22986 * t86873 * t30623;
    let t118640 = t1903 * t254;
    let t118649 = t23168 * t32789;
    let t118650 = 0.76763589786250567037e-1_f64 * t118649;
    let t118654 = 0.3289868133696452873e-1_f64 * t1888 * t23270 * t112873 * t1527;
    let t118661 = t23185 * t82074 * t32862;
    (t118639, t118640, t118650, t118654, t118661)
}
