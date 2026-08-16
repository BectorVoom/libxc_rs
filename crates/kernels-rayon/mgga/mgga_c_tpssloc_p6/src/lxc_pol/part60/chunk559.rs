//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 559/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk559(t1999: f64, t6600: f64, t6932: f64, t1996: f64, t6604: f64) -> (f64, f64, f64, f64) {
    let t6933 = t6600 * t1999;
    let t6934 = t6932 * t6933;
    let t6935 = 0.33643963411783659045e-4_f64 * t6934;
    let t6936 = t1996 * t6604;
    (t6933, t6934, t6935, t6936)
}
