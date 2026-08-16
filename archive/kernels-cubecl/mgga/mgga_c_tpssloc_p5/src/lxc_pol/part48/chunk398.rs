//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 398/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk398<F: Float>(t2684: F, t819: F, t820: F, t20: F, t61: F, t241: F, t244: F, t248: F, t238: F, t835: F, t841: F, t812: F) -> (F, F, F, F, F, F) {
    let t2686 = t819 * t820 * t2684;
    let t2690 = F::cast_from(1.0_f64) / t61 / t20;
    let t2691 = t2690 * t241;
    let t2693 = t2691 * t244 * t248;
    let t2695 = F::cast_from(119.0_f64) / F::cast_from(13824.0_f64) * t238 * t2693;
    let t2696 = t841 * t835;
    let t2697 = t812 * t2696;
    (t2686, t2690, t2691, t2693, t2695, t2697)
}
