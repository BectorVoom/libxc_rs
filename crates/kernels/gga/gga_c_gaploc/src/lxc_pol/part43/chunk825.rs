//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 825/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk825<F: Float>(t47130: F, t5241: F, t5640: F, t590: F, t1890: F, t1966: F, t13847: F, t825: F, t826: F, t12161: F, t123: F, t883: F, t2684: F, t2685: F, t12213: F, t2464: F, t2465: F) -> (F, F, F, F, F, F) {
    let t47133 = t5640 * t5241 * t47130 * t590;
    let t47137 = t1966 * t1890 * t47130 * t590;
    let t47140 = t825 * t826 * t13847;
    let t47143 = t12161 * t123 * t883;
    let t47145 = t2684 * t2685 * t47143;
    let t47149 = t2684 * t2464 * t2465 * t12213;
    (t47133, t47137, t47140, t47143, t47145, t47149)
}
