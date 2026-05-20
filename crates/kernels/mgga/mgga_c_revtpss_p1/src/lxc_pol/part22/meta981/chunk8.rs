//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3319/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3319<F: Float>(t14563: F, t14568: F, t14598: F, t14600: F, t4423: F, t676: F, t14602: F, t2482: F, t2811: F, t6016: F, t10535: F, t136: F, t2457: F, t6017: F) -> (F, F, F, F) {
    let t62983 = t14568 * t14563;
    let t62987 = t14598 * t14600 * t676 * t4423;
    let t62992 = t2482 * t2811 * t6016 * t14602;
    let t62999 = t10535 * t6017 * t136 * t2457;
    (t62983, t62987, t62992, t62999)
}
