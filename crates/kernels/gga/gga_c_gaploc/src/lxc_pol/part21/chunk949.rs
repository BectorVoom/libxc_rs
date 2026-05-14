//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 949/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk949<F: Float>(t10007: F, t5397: F, t15478: F, t1964: F, t10012: F, t169: F, t5750: F, t1234: F, t1683: F, t5335: F, t5344: F, t92: F, t291: F, t293: F, t5343: F, t539: F, t835: F) -> (F, F, F, F, F, F, F, F, F) {
    let t15483 = t10007 * t5397;
    let t15488 = t1964 * t15478;
    let t15490 = t10012 * t5397;
    let t15499 = t169 * t5750;
    let t15660 = t1234 * t1234;
    let t15665 = 1.0 / t5335 / t1683;
    let t15667 = t15665 * t92 * t5344;
    let t15672 = t291 / t5343 / t293;
    let t16036 = t539 * t835;
    (t15483, t15488, t15490, t15499, t15660, t15665, t15667, t15672, t16036)
}
