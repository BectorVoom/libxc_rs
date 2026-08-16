//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2687/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2687<F: Float>(t1062: F, t19857: F, t15745: F, t4845: F, t11859: F, t11922: F, t20074: F, t15926: F, t16035: F, t11927: F, t19830: F, t16055: F, t19738: F) -> (F, F, F, F, F, F) {
    let t67269 = t19857 * t1062;
    let t67301 = t15745 * t4845;
    let t67327 = t11859 * t11922 * t20074;
    let t67329 = t15926 * t16035;
    let t67353 = t11927 * t11922 * t19830;
    let t67355 = t19738 * t16055;
    (t67269, t67301, t67327, t67329, t67353, t67355)
}
