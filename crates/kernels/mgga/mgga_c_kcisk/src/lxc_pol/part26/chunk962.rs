//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 962/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk962<F: Float>(t1224: F, t4009: F, t7740: F, t1225: F, t25441: F, t25450: F, t6028: F, t7744: F, t25437: F, t4013: F, t25432: F, t25313: F, t13526: F, t13618: F, t20292: F, t20302: F, t20381: F, t20382: F, t26138: F, t26141: F, t26144: F, t26147: F) -> (F, F, F, F, F, F, F, F) {
    let t26150 = t1224 * t4009 * t7740;
    let t26153 = t1224 * t1225 * t25441;
    let t26156 = t1224 * t6028 * t25450;
    let t26159 = t1224 * t4009 * t7744;
    let t26162 = t1224 * t4013 * t25437;
    let t26165 = t1224 * t1225 * t25432;
    let t26168 = t1224 * t1225 * t25313;
    let t26170 = -t13618 - 4.0 / 27.0 * t13526 - 8.0 / 27.0 * t20292 + t20381 - t20382 + 4.0 / 9.0 * t20302 + 2.0 / 27.0 * t26138 - 10.0 / 27.0 * t26141 + 4.0 / 3.0 * t26144 - 8.0 / 9.0 * t26147 - 2.0 / 9.0 * t26150 - 2.0 * t26153 + 8.0 / 3.0 * t26156 + t26159 / 9.0 - 2.0 / 9.0 * t26162 + 2.0 / 3.0 * t26165 - t26168 / 3.0;
    (t26150, t26153, t26156, t26159, t26162, t26165, t26168, t26170)
}
