//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1196/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1196(t12079: f64, t18042: f64, t4435: f64, t1179: f64, t54391: f64, t15828: f64, t4450: f64, t1162: f64, t17903: f64, t2367: f64, t12869: f64, t18054: f64, t4464: f64) -> (f64, f64, f64, f64, f64) {
    let t55004 = t4435 * t12079 * t18042;
    let t55011 = t1179 * t54391;
    let t55021 = t4450 * t15828;
    let t55024 = t1162 * t2367 * t17903;
    let t55027 = t4464 * t12869 * t18054;
    (t55004, t55011, t55021, t55024, t55027)
}
