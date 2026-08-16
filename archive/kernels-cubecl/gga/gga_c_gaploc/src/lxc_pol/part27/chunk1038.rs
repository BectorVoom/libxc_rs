//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1038/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1038<F: Float>(t291: F, t293: F, t5343: F, t539: F, t835: F, t2086: F, t2109: F, t2102: F, t2154: F, t169: F, t4585: F, t2683: F, t5580: F) -> (F, F, F, F, F, F) {
    let t15672 = t291 / t5343 / t293;
    let t16036 = t539 * t835;
    let t16136 = t2109 * t2086;
    let t16239 = t2154 * t2102;
    let t16251 = t4585 * t169;
    let t16455 = t5580 * t2683;
    (t15672, t16036, t16136, t16239, t16251, t16455)
}
