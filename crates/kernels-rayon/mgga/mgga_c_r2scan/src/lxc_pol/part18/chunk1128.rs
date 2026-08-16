//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1128/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1128(t11618: f64, t11623: f64, t11631: f64, t11634: f64, t11637: f64, t12020: f64, t11858: f64, t39464: f64, t39470: f64, t39485: f64, t39558: f64, t39637: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t41107 = 5.0_f64 / 8.0_f64 * t11618;
    let t41108 = 45.0_f64 / 32.0_f64 * t11623;
    let t41109 = 5.0_f64 / 8.0_f64 * t11631;
    let t41110 = t11634 / 2.0_f64;
    let t41111 = 3.0_f64 / 2.0_f64 * t11637;
    let t41112 = 2.0_f64 * t12020;
    let t41113 = t11858 / 2.0_f64;
    let t41395 = 0.11902492299418487743e0_f64 * t39464;
    let t41397 = 0.28914548798370980346e-3_f64 * t39470;
    let t41405 = 0.93443229163669953711e-1_f64 * t39485;
    let t41439 = 0.45022119329691164871e0_f64 * t39558;
    let t41478 = 0.32927245914677557993e-1_f64 * t39637;
    (t41107, t41108, t41109, t41110, t41111, t41112, t41113, t41395, t41397, t41405, t41439, t41478)
}
