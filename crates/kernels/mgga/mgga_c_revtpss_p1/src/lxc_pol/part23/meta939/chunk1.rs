//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3086/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3086<F: Float>(t1134: F, t24317: F, t43821: F, t20356: F, t5079: F, t24312: F, t3390: F, t16857: F, t6449: F, t20337: F, t5071: F, t43946: F) -> (F, F, F, F, F, F) {
    let t81509 = t43821 * t24317 * t1134;
    let t81511 = t20356 * t5079;
    let t81513 = t3390 * t24312;
    let t81514 = t81513 * t1134;
    let t81516 = t16857 * t6449;
    let t81518 = t5071 * t20337;
    let t81521 = t43946 * t24317 * t1134;
    (t81509, t81511, t81514, t81516, t81518, t81521)
}
