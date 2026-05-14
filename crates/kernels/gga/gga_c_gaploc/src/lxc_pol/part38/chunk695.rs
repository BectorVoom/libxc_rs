//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 695/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk695<F: Float>(t30829: F, t31769: F, t544: F, t913: F, t10405: F, t2478: F, t6583: F, t3358: F, t6576: F, t3177: F, t8272: F, t9267: F, t12953: F, t4781: F, t34478: F, t9287: F) -> (F, F, F, F, F, F) {
    let t41884 = t544 * t30829 * t913 * t31769;
    let t41891 = t6583 * t10405 * t2478;
    let t41900 = t6576 * t3358 * t2478;
    let t41903 = t9267 * t8272 * t3177;
    let t41906 = t4781 * t12953;
    let t41909 = t544 * t34478 * t9287;
    (t41884, t41891, t41900, t41903, t41906, t41909)
}
