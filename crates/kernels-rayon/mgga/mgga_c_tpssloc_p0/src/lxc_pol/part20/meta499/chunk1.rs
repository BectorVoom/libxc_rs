//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2008/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2008(t1499: f64, t4280: f64, t3131: f64, t4649: f64, t1539: f64, t6733: f64, t3508: f64, t5011: f64, t1441: f64, t671: f64, t1388: f64, t1799: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17034 = t1499 * t4280;
    let t17732 = t3131 * t4649;
    let t17748 = t6733 * t1539;
    let t18946 = t3508 * t5011;
    let t19456 = t1441 * t671;
    let t19577 = t1799 * t1388;
    (t17034, t17732, t17748, t18946, t19456, t19577)
}
