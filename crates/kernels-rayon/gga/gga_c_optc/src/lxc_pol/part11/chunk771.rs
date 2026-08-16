//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 771/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk771(t1502: f64, t7274: f64, t1162: f64, t1179: f64, t12489: f64, t4434: f64, t7448: f64, t140: f64, t1514: f64, t2665: f64, t3183: f64, t3101: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12726 = t7274 * t1502;
    let t12727 = t1162 * t12726;
    let t12729 = t1179 * t12489;
    let t12741 = t4434 * t7448;
    let t12798 = t1514 * t2665 * t140;
    let t12799 = t3183 * t12798;
    let t12802 = t3101 * t12798;
    (t12726, t12727, t12729, t12741, t12799, t12802)
}
