//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 712/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk712<F: Float>(t33308: F, t9805: F, t9806: F, t15499: F, t28640: F, t3487: F, t2963: F, t3295: F, t9796: F, t1029: F, t9829: F, t3431: F, t5241: F, t2679: F, t20671: F, t28069: F, t33148: F) -> (F, F, F, F, F, F) {
    let t43403 = t9805 * t33308 * t9806;
    let t43407 = t28640 * t15499 * t3487 * t9806;
    let t43412 = t9796 * t2963 * t3295;
    let t43416 = t9796 * t1029 * t9829;
    let t43419 = t5241 * t3431;
    let t43421 = t9805 * t43419 * t2679;
    let t43425 = t28069 * t20671 * t33148;
    (t43403, t43407, t43412, t43416, t43421, t43425)
}
