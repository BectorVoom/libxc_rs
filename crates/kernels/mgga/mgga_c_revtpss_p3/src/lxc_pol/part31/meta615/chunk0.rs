//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2059/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2059<F: Float>(t14857: F, t25234: F, t25240: F, t2710: F, t4371: F, t10744: F, t4353: F, t7028: F, t14701: F, t92955: F, t241: F, t820: F, t93060: F) -> (F, F, F, F, F) {
    let t98972 = t25234 * t14857;
    let t98973 = F::cast_from(0.2032800112371413129e-3_f64) * t98972;
    let t98976 = t2710 * t25240 * t4371;
    let t98979 = t10744 * t7028 * t4353;
    let t98983 = t92955 * t14701;
    let t98984 = F::cast_from(0.2032800112371413129e-3_f64) * t98983;
    let t98988 = t820 * t93060 * t241;
    (t98973, t98976, t98979, t98984, t98988)
}
