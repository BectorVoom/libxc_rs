//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1528/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1528<F: Float>(t11922: F, t11927: F, t23838: F, t23998: F, t3115: F, t23916: F, t3091: F, t43131: F, t15618: F, t19785: F, t23820: F, t3153: F) -> (F, F, F, F, F) {
    let t78802 = t11927 * t11922 * t23838;
    let t78805 = t3115 * t11922 * t23998;
    let t78855 = t3091 * t43131 * t23916;
    let t78863 = t15618 * t19785;
    let t78873 = t23820 * t3153;
    (t78802, t78805, t78855, t78863, t78873)
}
