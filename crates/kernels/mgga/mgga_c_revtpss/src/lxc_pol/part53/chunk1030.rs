//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1030/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1030<F: Float>(t126078: F, t2747: F, t31767: F, t31772: F, t124: F, t1579: F, t800: F, t815: F, t32469: F, t32474: F, t119767: F, t1544: F, t247: F, t257: F, t837: F, t120046: F, t33721: F, t8486: F) -> (F, F, F, F, F) {
    let t126158 = t31767 * t2747 * t31772 * t126078;
    let t126163 = t815 * t800 * t124 * t1579;
    let t126164 = t32469 * t126163;
    let t126166 = t32474 * t126163;
    let t126182 = t119767 * t247 * t257 * t1544 * t837;
    let t126185 = t8486 * t120046 * t33721;
    (t126158, t126164, t126166, t126182, t126185)
}
