//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 825/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk825(t1873: f64, t88: f64, t2018: f64, t3701: f64, t2108: f64, t8301: f64, t2240: f64, t1874: f64, t7266: f64, t2165: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8601 = t88 * t1873;
    let t8643 = t3701 * t2018;
    let t8662 = t8301 * t2108;
    let t8663 = t2240 * t8662;
    let t8669 = t7266 * t1874;
    let t8675 = t2165 * t1873;
    (t8601, t8643, t8662, t8663, t8669, t8675)
}
