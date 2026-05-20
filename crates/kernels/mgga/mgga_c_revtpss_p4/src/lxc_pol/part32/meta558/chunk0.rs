//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1877/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1877<F: Float>(t1873: F, t94519: F, t94520: F, t94527: F, t94537: F, t94540: F, t26004: F, t5690: F, t13951: F, t2018: F, t807: F, t94565: F) -> (F, F, F, F, F, F, F, F) {
    let t98260 = t94519 * t1873;
    let t98263 = F::new(35.0) / F::new(108.0) * t94520;
    let t98264 = F::cast_from(0.1219527626469539185e-2_f64) * t94527;
    let t98267 = F::cast_from(0.10164000561857065645e-4_f64) * t94537;
    let t98268 = F::cast_from(0.72286371995927450868e-4_f64) * t94540;
    let t98269 = t26004 * t5690;
    let t98281 = t807 * t2018 * t13951;
    let t98283 = F::cast_from(0.18071592998981862717e-4_f64) * t94565;
    (t98260, t98263, t98264, t98267, t98268, t98269, t98281, t98283)
}
