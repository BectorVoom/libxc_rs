//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 459/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk459<F: Float>(t1152: F, t1157: F, t1392: F, t1430: F, t1442: F, t1835: F, t1839: F, t198: F, t4382: F, t4389: F, t446: F, t454: F, t5477: F, t5480: F, t589: F, t6017: F, t6020: F, t6031: F, t6034: F, t6039: F, t6067: F) -> (F,) {
    let t6070 = -0.32163648644302209643e2 * t6017 * t198 + 0.96490945932906628929e2 * t6020 * t446 + 0.19298189186581325786e3 * t5477 * t589 - 0.77192756746325303144e3 * t5480 * t1430 + 0.19298189186581325786e3 * t1442 * t1392 - 0.38596378373162651572e3 * t4382 * t1839 + 0.19298189186581325786e4 * t4389 * t6031 - 0.77192756746325303144e3 * t1157 * t6034 + 0.96490945932906628929e2 * t1152 * t1835 - 0.38596378373162651572e3 * t1157 * t6039 + 0.96490945932906628929e2 * t454 * t6067;
    (t6070,)
}
