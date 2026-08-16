//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1014/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1014<F: Float>(t33478: F, t34960: F, t1276: F, t1828: F, t247: F, t3719: F, t1811: F, t8937: F, t8945: F, t1769: F, t2150: F, t473: F) -> (F, F, F, F, F, F) {
    let t34961 = t33478 * t34960;
    let t34964 = t1276 * t1828;
    let t34966 = t247 * t3719 * t34964;
    let t34969 = t8937 * t1811;
    let t34972 = t34969 * t8945;
    let t34982 = t2150 * t473 * t1769;
    (t34961, t34964, t34966, t34969, t34972, t34982)
}
