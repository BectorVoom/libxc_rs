//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1155/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1155<F: Float>(t1731: F, t5304: F, t1730: F, t1773: F, t5255: F, t173: F, t1764: F, t614: F, t1736: F, t1732: F, t6895: F, t167: F, t168: F, t16942: F, t180: F, t66: F) -> (F, F, F, F, F, F, F, F) {
    let t17033 = t1731 * t5304;
    let t17034 = t1730 * t17033;
    let t17043 = t1730 * t5255 * t1773;
    let t17051 = t1764 * t173;
    let t17053 = t1730 * t17051 * t614;
    let t17054 = t17053 * t1736;
    let t17067 = t6895 * t1732;
    let t17088 = 0.28974367305964659283e0 * t167 * t168 / t66 / t16942 * t180;
    (t17033, t17034, t17043, t17051, t17053, t17054, t17067, t17088)
}
