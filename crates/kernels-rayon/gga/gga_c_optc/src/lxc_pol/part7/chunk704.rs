//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 704/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk704(t2229: f64, t758: f64, t1972: f64, t1975: f64, t1872: f64, t544: f64, t2204: f64, t732: f64, t43: f64, t97: f64, t1884: f64, t549: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6702 = t2229 * t758;
    let t6704 = t1972 * t1975;
    let t6709 = 12.0_f64 * t544 * t1872;
    let t6711 = 35.0_f64 / 3.0_f64 * t732 * t2204;
    let t6713 = 1.0_f64 / t97 / t43;
    let t6716 = t1884 * t549;
    (t6702, t6704, t6709, t6711, t6713, t6716)
}
