//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1152/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1152(t794: f64, t8537: f64, t6562: f64, t23237: f64, t8547: f64, t1880: f64, t2053: f64, t2717: f64) -> (f64, f64, f64, f64, f64) {
    let t31319 = t794 * t8537;
    let t31320 = t6562 * t31319;
    let t31321 = 0.41123351671205660912e-2_f64 * t31320;
    let t31329 = t23237 * t8547;
    let t31330 = t1880 * t31329;
    let t31332 = t2717 * t2053;
    (t31319, t31321, t31329, t31330, t31332)
}
