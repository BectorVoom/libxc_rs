//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1987/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1987(t87612: f64, t87618: f64, t87653: f64, t13263: f64, t13336: f64, t13397: f64, t2051: f64, t2633: f64, t26656: f64, t2684: f64, t4281: f64, t4291: f64, t81697: f64, t81704: f64, t87615: f64, t87627: f64, t87630: f64, t87633: f64, t87635: f64, t87640: f64, t87645: f64, t87650: f64) -> f64 {
    let t92760 = 0.3289868133696452873e-1_f64 * t87612;
    let t92768 = 0.3289868133696452873e-1_f64 * t87618;
    let t92781 = 0.16449340668482264365e-1_f64 * t87653;
    let t92782 = -t92760 + 0.9869604401089358619e-1_f64 * t87615 - 6.0_f64 * t13397 * t26656 * t13263 + 6.0_f64 * t4281 * t26656 * t2633 + t92768 - t4291 * t26656 * t2684 + t13336 * t2051 + 0.38381794893125283518e-1_f64 * t81697 - 0.16449340668482264365e-1_f64 * t87627 - 0.9869604401089358619e-1_f64 * t87630 + 0.3289868133696452873e-1_f64 * t87633 - 0.25587863262083522346e0_f64 * t87635 + 0.38381794893125283518e-1_f64 * t81704 + 0.9869604401089358619e-1_f64 * t87640 - 0.39478417604357434476e0_f64 * t87645 - 0.3289868133696452873e-1_f64 * t87650 - t92781;
    t92782
}
