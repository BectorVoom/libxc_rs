//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1465/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1465(t224: f64, t38891: f64, t38897: f64, t38906: f64, t39520: f64, t12326: f64, t617: f64, t12033: f64, t12036: f64, t31458: f64, t31461: f64, t31463: f64, t31465: f64, t31470: f64, t31472: f64, t31474: f64, t31476: f64, t31480: f64, t31483: f64, t32091: f64, t32093: f64, t32095: f64, t32099: f64, t38456: f64, t38458: f64, t38869: f64, t38872: f64, t38874: f64, t38876: f64) -> (f64, f64, f64, f64, f64) {
    let t39523 = t224 * (t38891 + t38897 + t38906 + t39520);
    let t39524 = t617 * t12326;
    let t39529 = 2.0_f64 * t12033;
    let t39530 = 4.0_f64 * t12036;
    let t39549 = -t31458 - t31461 - t31463 + t31465 + t38456 - t31470 + t31472 - t31474 + t31476 - t38458 - t38869 + t38872 + t31480 + t31483 - t32091 - t32093 + t38874 + t32095 + t32099 - t38876;
    (t39523, t39524, t39529, t39530, t39549)
}
