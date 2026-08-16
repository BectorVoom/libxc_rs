//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1104/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1104(t11603: f64, t5250: f64, t2839: f64, t5474: f64, t11927: f64, t4230: f64, t190: f64, t2837: f64, t5245: f64, t5243: f64, t11760: f64, t1570: f64) -> (f64, f64, f64, f64, f64) {
    let t43809 = t5250 * t11603;
    let t43834 = t5474 * t2839;
    let t43865 = t4230 * t11927;
    let t43891 = t2837 * t190 * t5245;
    let t43892 = t5243 * t43891;
    let t43906 = t1570 * t11760;
    (t43809, t43834, t43865, t43892, t43906)
}
