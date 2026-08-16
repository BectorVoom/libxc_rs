//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1685/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1685(t107: f64, t240: f64, t625: f64, t656: f64, t666: f64, t2331: f64, t63: f64, t43: f64, t614: f64, t2267: f64, t38: f64, t33: f64, t6504: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t22468 = t240 * t107;
    let t22469 = 11.0_f64 / 9.0_f64 * t22468;
    let t22470 = t625 * t656;
    let t22471 = t22470 * t666;
    let t22473 = t63 * t2331;
    let t22502 = t614 * t43;
    let t22505 = t38 * t2267;
    let t22510 = 88.0_f64 / 9.0_f64 * t240;
    let t22522 = t33 * t6504;
    (t22469, t22470, t22471, t22473, t22502, t22505, t22510, t22522)
}
