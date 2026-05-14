//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1146/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1146<F: Float>(t11937: F, t25500: F, t1024: F, t25553: F, t25495: F, t3215: F, t11817: F, t7117: F, t3223: F, t7125: F, t11940: F, t1972: F, t3204: F, t11788: F, t11782: F, t1007: F, t25532: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t93713 = t25500 * t11937;
    let t93715 = t1024 * t25553;
    let t93718 = t25495 * t3215;
    let t93720 = t7117 * t11817;
    let t93722 = t3223 * t7125;
    let t93725 = t11940 * t1972;
    let t93728 = t3204 * t7125;
    let t93731 = t11788 * t1972;
    let t93736 = t11782 * t1972;
    let t93743 = t25532 * t1007;
    (t93713, t93715, t93718, t93720, t93722, t93725, t93728, t93731, t93736, t93743)
}
