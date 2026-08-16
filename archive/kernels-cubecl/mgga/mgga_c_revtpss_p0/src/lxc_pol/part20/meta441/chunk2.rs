//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1677/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1677<F: Float>(t3552: F, t3781: F, t1204: F, t13147: F, t13141: F, t3596: F, t42859: F, t460: F, t3603: F, t43351: F, t1214: F, t17703: F) -> (F, F, F, F, F, F) {
    let t45764 = t3552 * t3781;
    let t45769 = t1204 * t13147;
    let t45779 = t1204 * t13141;
    let t45785 = t42859 * t3596;
    let t45786 = t460 * t45785;
    let t45787 = t43351 * t3603;
    let t45796 = t17703 * t1214;
    (t45764, t45769, t45779, t45786, t45787, t45796)
}
