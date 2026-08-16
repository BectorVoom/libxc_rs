//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1461/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1461(t224: f64, t38891: f64, t38897: f64, t38906: f64, t39520: f64, t12033: f64, t12036: f64, t12326: f64, t856: f64, t2255: f64, t31458: f64, t31461: f64, t31463: f64, t31465: f64, t31470: f64, t31472: f64, t31474: f64, t31476: f64, t31480: f64, t31483: f64, t32091: f64, t32093: f64, t32095: f64, t3751: f64, t38456: f64, t38458: f64, t38869: f64, t38872: f64, t38874: f64) -> (f64, f64, f64, f64, f64) {
    let t39523 = t224 * (t38891 + t38897 + t38906 + t39520);
    let t39529 = 2.0_f64 * t12033;
    let t39530 = 4.0_f64 * t12036;
    let t39551 = t856 * t12326;
    let t39577 = t2255 * t3751 - t31458 - t31461 - t31463 + t31465 - t31470 + t31472 - t31474 + t31476 + t31480 + t31483 - t32091 - t32093 + t32095 + t38456 - t38458 - t38869 + t38872 + t38874;
    (t39523, t39529, t39530, t39551, t39577)
}
