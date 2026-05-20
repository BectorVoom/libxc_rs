//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2569/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2569<F: Float>(t1284: F, t17288: F, t3624: F, t1260: F, t17289: F, t13032: F, t17524: F, t12881: F, t5381: F, t17861: F, t17416: F, t3647: F) -> (F, F, F, F, F, F) {
    let t57040 = t17288 * t1284 * t3624;
    let t57053 = t17289 * t1260;
    let t57056 = t13032 * t17524;
    let t57094 = t5381 * t12881;
    let t57100 = t17861 * t3624;
    let t57118 = t3647 * t17416;
    (t57040, t57053, t57056, t57094, t57100, t57118)
}
