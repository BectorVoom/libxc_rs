//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2753/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2753<F: Float>(t10963: F, t9303: F, t10069: F, t10934: F, t10518: F, t10542: F, t10612: F, t2398: F, t2434: F, t2626: F, t2629: F, t676: F, t9425: F) -> (F, F, F, F, F, F, F) {
    let t39724 = t9303 * t10963;
    let t39726 = t10069 * t10934;
    let t39731 = t10542 * t10518;
    let t39737 = t2398 * t10612;
    let t39739 = t2434 * t2626;
    let t39741 = F::cast_from(0.86748650402413918736e-1_f64) * t2629 * t39739;
    let t39742 = t676 * t9425;
    (t39724, t39726, t39731, t39737, t39739, t39741, t39742)
}
