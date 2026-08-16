//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1553/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1553<F: Float>(t1261: F, t12884: F, t24232: F, t247: F, t1263: F, t24616: F, t24633: F, t17525: F, t21188: F, t24758: F, t3172: F, t3711: F) -> (F, F, F, F, F) {
    let t82757 = t1261 * t247 * t12884 * t24232;
    let t82799 = t1263 * t24616;
    let t82816 = t1263 * t24633;
    let t82821 = t17525 * t21188;
    let t82824 = t3711 * t3172 * t24758;
    (t82757, t82799, t82816, t82821, t82824)
}
