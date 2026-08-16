//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 777/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk777(t2679: f64, t7354: f64, t2684: f64, t1844: f64, t2465: f64, t2464: f64, t825: f64, t6125: f64, t549: f64, t7222: f64, t2021: f64, t2026: f64) -> (f64, f64, f64, f64, f64) {
    let t7355 = t7354 * t2679;
    let t7356 = t2684 * t7355;
    let t7358 = t2465 * t1844;
    let t7359 = t2464 * t7358;
    let t7360 = t825 * t7359;
    let t7362 = t2465 * t6125;
    let t7363 = t2464 * t7362;
    let t7364 = t2684 * t7363;
    let t7366 = t549 * t7222;
    let t7371 = t2021 * t2026;
    (t7356, t7360, t7364, t7366, t7371)
}
