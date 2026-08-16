//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2195/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2195<F: Float>(t4173: F, t4187: F, t21698: F, t603: F, t5816: F, t640: F, t77: F, t29561: F, t644: F, t4241: F, t7705: F, t1927: F) -> (F, F, F, F, F, F) {
    let t108813 = t4173 * t4187;
    let t108816 = t603 * t21698;
    let t108864 = t77 * t640 * t5816;
    let t108872 = t77 * t29561 * t644;
    let t108876 = t77 * t7705 * t4241;
    let t108879 = t1927 * t5816;
    (t108813, t108816, t108864, t108872, t108876, t108879)
}
