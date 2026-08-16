//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1054/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1054(t2034: f64, t22871: f64, t1948: f64, t6926: f64, t2035: f64, t6560: f64, t2067: f64, t6785: f64, t162: f64, t2017: f64, t6956: f64, t2003: f64, t2010: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t22872 = t2034 * t22871;
    let t22875 = t6926 * t1948;
    let t22876 = t2034 * t22875;
    let t22879 = t2035 * t6560;
    let t22880 = t2034 * t22879;
    let t22883 = t6785 * t2067;
    let t22884 = t162 * t22883;
    let t22887 = t6956 * t2017;
    let t22889 = t2003 * t2010;
    (t22872, t22875, t22876, t22879, t22880, t22883, t22884, t22887, t22889)
}
