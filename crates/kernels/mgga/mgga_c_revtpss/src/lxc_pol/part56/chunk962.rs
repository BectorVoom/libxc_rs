//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 962/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk962<F: Float>(t1263: F, t494: F, t1122: F, t32015: F, t1276: F, t1294: F, t247: F, t3719: F, t1209: F, t8931: F, t7642: F, t2142: F, t2148: F) -> (F, F, F, F, F, F, F, F) {
    let t33426 = t1263 * t494;
    let t33427 = t33426 * t1122;
    let t33428 = t32015 * t33427;
    let t33431 = t1276 * t1294;
    let t33433 = t247 * t3719 * t33431;
    let t33436 = t1209 * t8931;
    let t33441 = t7642 * t8931;
    let t33446 = t2148 * t2142;
    (t33426, t33427, t33428, t33431, t33433, t33436, t33441, t33446)
}
