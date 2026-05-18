//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1061/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1061<F: Float>(t10782: F, t2464: F, t2465: F, t2684: F, t13072: F, t32757: F, t25359: F, t2615: F, t9438: F, t1445: F, t3209: F, t833: F, t8469: F) -> (F, F, F, F) {
    let t44128 = t2684 * t2464 * t2465 * t10782;
    let t44130 = t32757 * t13072;
    let t44131 = F::new(0.89376224879626066675e-1) * t44130;
    let t44133 = t2615 * t9438 * t25359;
    let t44134 = F::new(0.15976219147466979032e-1) * t44133;
    let t44138 = F::new(0.43710935587469654631e2) * t833 * t1445 * t8469 * t3209;
    (t44128, t44131, t44134, t44138)
}
