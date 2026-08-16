//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 1258/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk1258(t1062: f64, t3728: f64, t6935: f64, t2209: f64, t3739: f64, t24081: f64, t6853: f64, t22851: f64, t6181: f64, t10346: f64, t11210: f64, t16677: f64, t19: f64, t6939: f64) -> (f64, f64, f64, f64, f64) {
    let t35809 = t1062 * t3728 * t6935;
    let t35811 = t2209 * t3739;
    let t35813 = t24081 * t6853;
    let t35815 = t35813 * t6181 * t22851;
    let t35820 = t10346 * t6939 * t19 * t11210 * t16677;
    (t35809, t35811, t35813, t35815, t35820)
}
