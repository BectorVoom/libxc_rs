//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1166/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1166(t31585: f64, t6508: f64, t1358: f64, t6507: f64, t2293: f64, t986: f64) -> (f64, f64, f64) {
    let t31586 = t6508 * t31585;
    let t31589 = 0.12646669615856066488e-1_f64 * t1358 * t6507 * t31586;
    let t31590 = t986 * t2293;
    (t31586, t31589, t31590)
}
