//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1276/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1276<F: Float>(t1610: F, t32512: F, t15093: F, t2744: F, t55867: F, t9425: F, t21499: F, t32025: F, t32042: F, t32096: F, t13820: F, t9445: F, t53214: F, t9446: F, t9453: F, t20160: F, t32014: F) -> (F, F, F, F, F, F, F, F) {
    let t110126 = t32512 * t1610;
    let t110136 = t2744 * t15093;
    let t110219 = t9425 * t55867;
    let t110222 = t32025 * t21499;
    let t110242 = t32096 * t32042;
    let t110244 = t9445 * t13820;
    let t110256 = t9446 * t53214 * t9453;
    let t110261 = t9446 * t20160 * t32014;
    (t110126, t110136, t110219, t110222, t110242, t110244, t110256, t110261)
}
