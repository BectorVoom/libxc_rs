//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1123/1445 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1123<F: Float>(t1038: F, t11589: F, t147: F, t19509: F, t457: F, t137: F, t27144: F, t1552: F, t3143: F, t674: F, t1666: F, t3074: F, t4: F, t5216: F) -> (F, F, F, F) {
    let t27940 = t11589 * t1038 * t19509 * t147 * t457;
    let t28006 = t27144 * t137;
    let t28065 = M_PI * t1552 * t674 * t3143;
    let t28169 = t1666 * t3074 * t5216 * t4;
    (t27940, t28006, t28065, t28169)
}
