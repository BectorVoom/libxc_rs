//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 953/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk953<F: Float>(t7315: F, t8714: F, t2014: F, t7239: F, t8698: F, t7235: F, t8715: F, t2022: F, t7506: F, t8707: F, t2097: F, t7274: F, t32287: F, t32266: F, t32270: F, t1444: F, t8708: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t32662 = t8714 * t7315;
    let t32663 = t2014 * t32662;
    let t32667 = 3.0 * t8698 * t7239;
    let t32671 = t7235 * t8715;
    let t32673 = t7506 * t2022;
    let t32674 = t8707 * t32673;
    let t32677 = t2097 * t7274;
    let t32678 = t8707 * t32677;
    let t32681 = 0.17354086964223805049e-2 * t32287;
    let t32682 = 0.3718732920905101082e-4 * t32266;
    let t32683 = 0.66119071333692697238e-4 * t32270;
    let t32685 = t8708 * t1444;
    (t32662, t32663, t32667, t32671, t32673, t32674, t32677, t32678, t32681, t32682, t32683, t32685)
}
