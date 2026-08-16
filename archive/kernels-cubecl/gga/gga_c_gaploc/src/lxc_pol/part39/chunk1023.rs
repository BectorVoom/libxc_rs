//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1023/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1023<F: Float>(t326: F, t43508: F, t825: F, t2684: F, t7585: F, t43490: F, t13145: F, t2033: F, t549: F, t33360: F, t787: F, t9824: F) -> (F, F, F, F, F) {
    let t43511 = F::cast_from(0.92023022289409799224e1_f64) * t825 * t326 * t43508;
    let t43514 = F::cast_from(0.43710935587469654631e2_f64) * t2684 * t7585 * t43508;
    let t43516 = t2684 * t7585 * t43490;
    let t43519 = t2033 * t549 * t13145;
    let t43522 = t787 * t33360 * t9824;
    (t43511, t43514, t43516, t43519, t43522)
}
