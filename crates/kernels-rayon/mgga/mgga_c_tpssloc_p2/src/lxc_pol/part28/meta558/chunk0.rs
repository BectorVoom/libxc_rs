//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1829/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1829(t1404: f64, t7222: f64, t24447: f64, t580: f64, t2098: f64, t3946: f64, t1395: f64, t7240: f64, t1453: f64, t81439: f64, t26129: f64, t81442: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t85381 = t7222 * t1404;
    let t85392 = t24447 * t580;
    let t85394 = t2098 * t3946;
    let t85397 = t1395 * t7240;
    let t86586 = t81439 * t1453;
    let t86588 = t81442 * t26129;
    (t85381, t85392, t85394, t85397, t86586, t86588)
}
