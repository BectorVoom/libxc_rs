//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3612/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3612<F: Float>(t3391: F, t43821: F, t6442: F, t12327: F, t6449: F, t43946: F, t12331: F, t16926: F, t5071: F, t1134: F, t20337: F, t3390: F) -> (F, F, F, F, F, F) {
    let t68470 = t43821 * t6442 * t3391;
    let t68473 = t12327 * t6449 * t3391;
    let t68476 = t43946 * t6442 * t3391;
    let t68479 = t12331 * t6449 * t3391;
    let t68481 = t5071 * t16926;
    let t68484 = t3390 * t20337 * t1134;
    (t68470, t68473, t68476, t68479, t68481, t68484)
}
