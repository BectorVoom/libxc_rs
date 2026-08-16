//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1044/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1044<F: Float>(t11704: F, t2251: F, t3109: F, t828: F, t3096: F, t3091: F, t1020: F, t3105: F, t247: F, t2862: F, t1063: F, t126: F, t3181: F) -> (F, F, F, F, F, F) {
    let t11705 = t11704 * t2251;
    let t11710 = t828 * t3109;
    let t11711 = t11710 * t3096;
    let t11712 = t3091 * t11711;
    let t11714 = t1020 * t3105;
    let t11722 = t247 * t3109 * t2862;
    let t11723 = t1063 * t11722;
    let t11725 = t126 * t3181;
    (t11705, t11710, t11712, t11714, t11723, t11725)
}
