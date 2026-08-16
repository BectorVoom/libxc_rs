//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2019/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2019<F: Float>(t22935: F, t6883: F, t22667: F, t1987: F, t81144: F, t9537: F, t107: F, t835: F, t240: F, t656: F, t666: F, t2331: F, t625: F) -> (F, F, F, F, F, F, F) {
    let t81393 = t6883 * t22935;
    let t81395 = t6883 * t22667;
    let t81398 = t81144 * t9537 * t1987;
    let t81399 = F::cast_from(0.13707783890401886971e-2_f64) * t81398;
    let t81437 = t835 * t107;
    let t81438 = F::cast_from(154.0_f64) / F::cast_from(27.0_f64) * t81437;
    let t81439 = t240 * t656;
    let t81440 = t81439 * t666;
    let t81442 = t625 * t2331;
    (t81393, t81395, t81399, t81438, t81439, t81440, t81442)
}
