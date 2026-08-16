//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1511/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1511<F: Float>(t4343: F, t854: F, t236: F, t807: F, t124: F, t14468: F, t800: F, t775: F) -> (F, F, F, F) {
    let t14741 = t854 * t4343;
    let t14742 = t236 * t14741;
    let t14744 = F::cast_from(0.57165357490759649296e-4_f64) * t807 * t14742;
    let t14745 = t124 * t14468;
    let t14746 = t800 * t14745;
    let t14749 = t4343 * t775;
    (t14741, t14744, t14746, t14749)
}
