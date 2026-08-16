//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 931/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk931(t1369: f64, t3866: f64, t1995: f64, t241: f64, t67: f64, t3734: f64, t820: f64) -> (f64, f64, f64) {
    let t3867 = t3866 * t1369;
    let t3869 = t241 * t1995;
    let t3870 = t3869 * t67;
    let t3872 = t3870 * t820 * t3734;
    (t3867, t3870, t3872)
}
