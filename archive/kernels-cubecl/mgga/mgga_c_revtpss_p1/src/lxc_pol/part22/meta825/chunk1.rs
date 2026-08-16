//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2944/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2944<F: Float>(t3857: F, t5567: F, t1317: F, t13672: F, t2608: F, t512: F, t5566: F, t1856: F, t9544: F, t13597: F, t2516: F, t2626: F) -> (F, F, F, F, F, F) {
    let t48235 = t3857 * t5567;
    let t48237 = t1317 * t13672;
    let t48240 = t512 * t5566 * t2608;
    let t48243 = t512 * t1856 * t9544;
    let t48255 = t13597 * t2516;
    let t48260 = t13597 * t2626;
    (t48235, t48237, t48240, t48243, t48255, t48260)
}
