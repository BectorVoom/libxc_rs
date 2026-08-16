//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1390/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1390(t3200: f64, t3217: f64, t3219: f64, t1150: f64, t1152: f64, t3902: f64, t1170: f64, t2586: f64, t9030: f64, t115: f64, t25834: f64, t426: f64) -> (f64, f64, f64, f64) {
    let t27687 = t3217 * t3200 * t3219;
    let t27699 = t1150 * t3902 * t1152;
    let t27702 = t1170 * t2586 * t9030;
    let t27705 = t426 * t25834 * t115;
    (t27687, t27699, t27702, t27705)
}
