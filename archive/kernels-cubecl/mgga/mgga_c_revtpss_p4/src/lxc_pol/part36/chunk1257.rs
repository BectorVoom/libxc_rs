//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1257/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1257<F: Float>(t25305: F, t99380: F, t2453: F, t2458: F, t7760: F, t25331: F, t27213: F, t93190: F, t99211: F, t25374: F, t98848: F, t99403: F) -> (F, F, F, F, F, F) {
    let t99425 = t25305 * t99380;
    let t99435 = t2453 * t7760 * t2458;
    let t99456 = t27213 * t25331;
    let t99460 = t93190 * t99211;
    let t99463 = t98848 * t25374;
    let t99466 = t99403 * t25374;
    (t99425, t99435, t99456, t99460, t99463, t99466)
}
