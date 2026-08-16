//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1313/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1313(t41: f64, t42: f64, t53: f64, t54: f64, t2585: f64, t2769: f64, t73: f64, t3241: f64, t76: f64, t111: f64, t2311: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9287 = 1.0_f64 / t42 / t41;
    let t9300 = 1.0_f64 / t54 / t53;
    let t9311 = 1232.0_f64 / 27.0_f64 * t2585;
    let t9321 = 1.0_f64 / t73 / t2769;
    let t9330 = 1.0_f64 / t76 / t3241;
    let t9348 = t2311 * t111;
    (t9287, t9300, t9311, t9321, t9330, t9348)
}
