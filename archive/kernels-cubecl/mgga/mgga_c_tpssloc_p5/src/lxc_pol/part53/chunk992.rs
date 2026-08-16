//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 992/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk992<F: Float>(t1888: F, t232: F, t6646: F, t92552: F, t26676: F, t33384: F, t6547: F, t121495: F, t25038: F, t25248: F, t776: F, t33429: F) -> (F, F, F, F, F) {
    let t121560 = t1888 * t6646 * t92552 * t232;
    let t121563 = t1888 * t6646 * t26676;
    let t121574 = t6547 * t33384;
    let t121612 = t25038 * t25248 * t121495 * t776;
    let t121629 = t6547 * t33429;
    (t121560, t121563, t121574, t121612, t121629)
}
