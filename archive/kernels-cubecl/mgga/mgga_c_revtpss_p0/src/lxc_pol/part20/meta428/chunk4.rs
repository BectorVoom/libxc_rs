//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1611/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1611<F: Float>(t44306: F, t44319: F, t459: F, t1256: F, t12890: F, t3588: F, t482: F, t1222: F, t3693: F, t697: F, t13021: F, t140: F) -> (F, F, F, F, F, F) {
    let t44321 = (t44306 + t44319) * t459;
    let t44326 = t12890 * t1256;
    let t44332 = t3588 * t3588;
    let t44333 = t482 * t44332;
    let t44343 = t1222 * t697 * t3693;
    let t44346 = t1222 * t140 * t13021;
    (t44321, t44326, t44332, t44333, t44343, t44346)
}
