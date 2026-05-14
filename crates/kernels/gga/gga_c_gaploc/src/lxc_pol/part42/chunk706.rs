//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 706/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk706<F: Float>(t33289: F, t9800: F, t9806: F, t11068: F, t2679: F, t9796: F, t33308: F, t9805: F, t15499: F, t28640: F, t3487: F, t2963: F, t3295: F, t1029: F, t9829: F, t20671: F, t28069: F, t33148: F) -> (F, F, F, F, F, F, F) {
    let t43389 = t9800 * t33289 * t9806;
    let t43400 = t9796 * t11068 * t2679;
    let t43403 = t9805 * t33308 * t9806;
    let t43407 = t28640 * t15499 * t3487 * t9806;
    let t43412 = t9796 * t2963 * t3295;
    let t43416 = t9796 * t1029 * t9829;
    let t43425 = t28069 * t20671 * t33148;
    (t43389, t43400, t43403, t43407, t43412, t43416, t43425)
}
