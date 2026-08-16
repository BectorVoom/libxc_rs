//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2048/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2048<F: Float>(t1989: F, t41937: F, t1113: F, t2411: F, t26088: F, t531: F, t2470: F, t26049: F, t7284: F, t2453: F, t555: F, t25898: F) -> (F, F, F, F, F, F, F) {
    let t94149 = t1989 * t41937;
    let t94245 = t2411 * t1113;
    let t94358 = t531 * t26088;
    let t94377 = t26049 * t2470;
    let t94378 = t7284 * t94377;
    let t94382 = t2453 * t555;
    let t94383 = t94382 * t25898;
    (t94149, t94245, t94358, t94377, t94378, t94382, t94383)
}
