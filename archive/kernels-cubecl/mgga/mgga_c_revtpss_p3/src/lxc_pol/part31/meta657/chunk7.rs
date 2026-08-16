//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2221/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2221<F: Float>(t21881: F, t94: F, t1937: F, t29508: F, t6993: F, t25082: F, t86815: F, t8717: F, t7003: F, t27123: F, t7735: F, t27126: F) -> (F, F, F, F, F, F) {
    let t108714 = t94 * t21881;
    let t108716 = F::cast_from(2.0_f64) * t108714 * t1937;
    let t108718 = F::cast_from(2.0_f64) * t29508 * t6993;
    let t108721 = F::cast_from(6.0_f64) * t25082 * t8717 * t86815;
    let t108723 = F::cast_from(2.0_f64) * t29508 * t7003;
    let t108725 = F::cast_from(4.0_f64) * t27123 * t7735;
    let t108727 = F::cast_from(4.0_f64) * t27126 * t7735;
    (t108716, t108718, t108721, t108723, t108725, t108727)
}
