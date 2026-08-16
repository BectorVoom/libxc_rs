//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1378/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1378(t33746: f64, t7000: f64, t2314: f64, t33726: f64, t19456: f64, t8675: f64, t31908: f64, t4028: f64, t4034: f64, t652: f64, t7408: f64, t7467: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t123119 = t33746 * t7000;
    let t123120 = t2314 * t33726;
    let t123122 = t19456 * t8675;
    let t123124 = t4028 * t31908;
    let t123126 = t4034 * t33726;
    let t123129 = t652 * t7408 * t7467;
    (t123119, t123120, t123122, t123124, t123126, t123129)
}
