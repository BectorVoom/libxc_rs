//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1227/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1227(t10961: f64, t2197: f64, t10713: f64, t4614: f64, t833: f64, t24364: f64, t955: f64, t16136: f64, t3504: f64, t28387: f64, t3025: f64, t10627: f64, t1865: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t32771 = 0.30674340763136599742e2_f64 * t2197 * t10961;
    let t32774 = 0.30674340763136599742e2_f64 * t833 * t4614 * t10713;
    let t32778 = 0.79445533226334281487e-1_f64 * t955 * t24364;
    let t32785 = 0.69017266717057349418e1_f64 * t16136 * t3504;
    let t32791 = 0.10725146985555128001e1_f64 * t3025 * t28387;
    let t32803 = t10627 * t1865;
    (t32771, t32774, t32778, t32785, t32791, t32803)
}
