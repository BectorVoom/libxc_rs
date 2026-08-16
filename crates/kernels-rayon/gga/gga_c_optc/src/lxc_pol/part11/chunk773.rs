//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 773/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk773(t1533: f64, t3201: f64, t1107: f64, t1506: f64, t1570: f64, t2839: f64, t4580: f64, t544: f64, t539: f64, t172: f64, t4595: f64, t4611: f64, t740: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12864 = t1533 * t3201;
    let t12869 = t1107 * t1506;
    let t12943 = t1570 * t2839;
    let t12966 = t544 * t4580;
    let t12971 = t539 * t4580;
    let t12979 = t172 * t4595;
    let t12987 = t4611 * t740;
    (t12864, t12869, t12943, t12966, t12971, t12979, t12987)
}
