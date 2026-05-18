//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1465/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1465<F: Float>(t224: F, t38891: F, t38897: F, t38906: F, t39520: F, t12326: F, t617: F, t12033: F, t12036: F, t31458: F, t31461: F, t31463: F, t31465: F, t31470: F, t31472: F, t31474: F, t31476: F, t31480: F, t31483: F, t32091: F, t32093: F, t32095: F, t32099: F, t38456: F, t38458: F, t38869: F, t38872: F, t38874: F, t38876: F) -> (F, F, F, F, F) {
    let t39523 = t224 * (t38891 + t38897 + t38906 + t39520);
    let t39524 = t617 * t12326;
    let t39529 = F::new(2.0) * t12033;
    let t39530 = F::new(4.0) * t12036;
    let t39549 = -t31458 - t31461 - t31463 + t31465 + t38456 - t31470 + t31472 - t31474 + t31476 - t38458 - t38869 + t38872 + t31480 + t31483 - t32091 - t32093 + t38874 + t32095 + t32099 - t38876;
    (t39523, t39524, t39529, t39530, t39549)
}
