//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 2014/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2014(t90807: f64, t90837: f64, t93473: f64, t93476: f64, t93483: f64, t93488: f64, t93489: f64, t93490: f64, t93491: f64, t93494: f64, t96935: f64, t96937: f64, t96941: f64, t96945: f64, t96949: f64, t96954: f64, t96958: f64) -> f64 {
    let t102558 = 0.6579736267392905746e-1_f64 * t96935 - 0.76763589786250567037e-1_f64 * t96937 - t93473 + t93476 - 0.16449340668482264365e-1_f64 * t96941 + t93483 - t93488 + t93489 + t93490 + t93491 + t93494 - 0.5117572652416704469e0_f64 * t90807 + 0.38381794893125283518e-1_f64 * t96945 - 0.16449340668482264365e-1_f64 * t96949 + 0.9869604401089358619e-1_f64 * t96954 - 0.3289868133696452873e-1_f64 * t96958 - 0.20835831513410868196e0_f64 * t90837;
    t102558
}
