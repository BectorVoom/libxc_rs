//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3657/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3657<F: Float>(t16943: F, t5063: F, t43748: F, t6439: F, t12238: F, t6471: F, t20448: F, t3379: F, t1196: F, t3520: F, t3523: F, t68795: F) -> (F, F, F, F, F) {
    let t69101 = F::new(2.0) * t5063 * t16943;
    let t69103 = F::new(2.0) * t43748 * t6439;
    let t69105 = F::new(1.0) * t12238 * t6471;
    let t69107 = F::new(2.0) * t3379 * t20448;
    let t69111 = F::cast_from(0.34631718211362927518e2_f64) * t1196 * t3520 * t68795 * t3523;
    (t69101, t69103, t69105, t69107, t69111)
}
