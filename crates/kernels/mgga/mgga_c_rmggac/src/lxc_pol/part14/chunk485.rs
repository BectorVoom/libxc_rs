//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 485/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk485<F: Float>(t1144: F, t1152: F, t1157: F, t1392: F, t1430: F, t1442: F, t198: F, t4379: F, t4382: F, t4389: F, t446: F, t454: F, t5436: F, t5439: F, t5474: F, t5477: F, t5480: F, t5491: F, t5527: F, t589: F, t998: F) -> (F,) {
    let t5530 = -0.32163648644302209643e2 * t5474 * t198 + 0.19298189186581325786e3 * t5477 * t446 - 0.38596378373162651572e3 * t5480 * t1144 + 0.96490945932906628929e2 * t1442 * t998 + 0.96490945932906628929e2 * t4379 * t589 - 0.77192756746325303144e3 * t4382 * t1430 + 0.19298189186581325786e3 * t1152 * t1392 + 0.19298189186581325786e4 * t4389 * t5491 - 0.77192756746325303144e3 * t1157 * t5436 - 0.38596378373162651572e3 * t1157 * t5439 + 0.96490945932906628929e2 * t454 * t5527;
    (t5530,)
}
