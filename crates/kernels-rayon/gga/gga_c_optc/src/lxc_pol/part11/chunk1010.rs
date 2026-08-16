//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1010/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1010(t1757: f64, t535: f64, t6446: f64, t1835: f64, t209: f64, t6447: f64, t508: f64, t6451: f64, t6455: f64, t1810: f64, t10194: f64, t31: f64, t4: f64) -> (f64, f64, f64, f64, f64) {
    let t22403 = 8.0_f64 * t1757 * t535 * t6446;
    let t22406 = 0.71233333333333333333e-1_f64 * t209 * t1835 * t6447;
    let t22410 = 0.36845452142031360636e2_f64 * t209 * t508 * t6451 * t6455;
    let t22411 = t1810 * t1810;
    let t22417 = 0.11483710345679012345e-1_f64 * t4 * t10194 * t31;
    (t22403, t22406, t22410, t22411, t22417)
}
