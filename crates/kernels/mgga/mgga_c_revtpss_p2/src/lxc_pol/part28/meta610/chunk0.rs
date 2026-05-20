//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2131/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2131<F: Float>(t25082: F, t49582: F, t8717: F, t2014: F, t25089: F, t28172: F, t27154: F, t95088: F, t26089: F, t5542: F, t13425: F, t13537: F, t1843: F, t2007: F, t25096: F, t28025: F, t4246: F, t4293: F, t6985: F, t7221: F, t98426: F, t98428: F, t98430: F, t98432: F, t98439: F, t98440: F, t98442: F, t98449: F, t98452: F, t98455: F) -> F {
    let t98458 = F::new(3.0) * t25082 * t8717 * t49582;
    let t98461 = F::new(3.0) * t2014 * t28172 * t25089;
    let t98463 = F::new(6.0) * t95088 * t27154;
    let t98467 = t2014 * t26089 * t5542;
    let t98468 = -t13425 * t2007 - F::new(2.0) * t13537 * t6985 - F::new(2.0) * t1843 * t25096 - F::new(4.0) * t28025 * t4293 - F::new(2.0) * t4246 * t7221 - t98426 - t98428 - t98430 - t98432 - t98439 + t98440 - t98442 + t98449 - t98452 - t98455 - t98458 + t98461 - t98463 - t98467;
    t98468
}
