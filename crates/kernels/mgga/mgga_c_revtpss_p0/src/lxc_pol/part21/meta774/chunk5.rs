//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2755/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2755<F: Float>(t10811: F, t14793: F, t14774: F, t2652: F, t10726: F, t14860: F, t2661: F, t4366: F, t2662: F, t837: F, t2646: F, t4352: F) -> (F, F, F, F, F) {
    let t50722 = t10811 * t14793;
    let t50724 = t2652 * t14774;
    let t50728 = t2661 * t10726 * t14860 * t4366;
    let t50732 = t2661 * t2662 * t14860 * t837;
    let t50736 = t2661 * t2662 * t4352 * t2646;
    (t50722, t50724, t50728, t50732, t50736)
}
