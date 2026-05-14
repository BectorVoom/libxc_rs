//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 402/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk402<F: Float>(t1908: F, t2594: F, t1959: F, t2366: F, t1968: F, t1971: F, t2373: F, t2376: F, t2379: F, t1974: F) -> (F, F, F, F) {
    let t2595 = t1908 * t2594;
    let t2597 = -t1959 - 0.17123333333333333333e-1 * t2366;
    let t2604 = 0.3529725e1 * t2373 - t1968 - 0.516475e0 * t2366 + 0.6311625e0 * t2376 - t1971 - 0.104195e0 * t2379;
    let t2605 = t2604 * t1974;
    (t2595, t2597, t2604, t2605)
}
