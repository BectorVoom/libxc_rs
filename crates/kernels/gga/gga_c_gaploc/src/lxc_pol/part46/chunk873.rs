//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 873/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk873<F: Float>(t13072: F, t32757: F, t25359: F, t2615: F, t9438: F, t1445: F, t3209: F, t833: F, t8469: F, t25405: F, t5748: F, t13034: F, t15751: F, t10948: F, t9972: F, t41448: F) -> (F, F, F, F, F, F, F) {
    let t44130 = t32757 * t13072;
    let t44131 = 0.89376224879626066675e-1 * t44130;
    let t44133 = t2615 * t9438 * t25359;
    let t44134 = 0.15976219147466979032e-1 * t44133;
    let t44138 = 0.43710935587469654631e2 * t833 * t1445 * t8469 * t3209;
    let t44142 = 0.27606906686822939767e2 * t5748 * t1445 * t25405 * t3209;
    let t44144 = 0.27606906686822939767e2 * t15751 * t13034;
    let t44145 = t10948 * t9972;
    let t44147 = 0.31952438294933958063e0 * t41448;
    (t44131, t44134, t44138, t44142, t44144, t44145, t44147)
}
