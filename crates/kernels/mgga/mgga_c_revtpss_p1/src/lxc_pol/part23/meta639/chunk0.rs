//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2344/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2344<F: Float>(t10963: F, t9303: F, t2434: F, t2626: F, t2629: F, t676: F, t9425: F, t2567: F, t2576: F, t2582: F) -> (F, F, F, F, F, F) {
    let t39724 = t9303 * t10963;
    let t39739 = t2434 * t2626;
    let t39741 = F::cast_from(0.86748650402413918736e-1_f64) * t2629 * t39739;
    let t39742 = t676 * t9425;
    let t39744 = F::cast_from(0.1301229756036208781e0_f64) * t2629 * t39742;
    let t39747 = F::new(36.0) * t2582 * t2567 * t2576;
    (t39724, t39739, t39741, t39742, t39744, t39747)
}
