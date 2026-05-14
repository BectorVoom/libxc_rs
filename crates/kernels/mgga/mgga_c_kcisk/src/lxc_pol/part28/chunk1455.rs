//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1455/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1455<F: Float>(t111582: F, t111583: F, t111584: F, t111585: F, t116064: F, t118621: F, t120929: F, t35552: F, t35554: F, t35556: F, t35559: F, t109148: F, t109149: F, t111577: F, t116059: F, t116061: F, t116062: F, t116063: F, t123493: F, t32686: F, t35547: F, t35550: F) -> (F,) {
    let t123495 = -t116064 + t35552 + t120929 + t35554 + t35556 + t111582 - t35559 - t111583 - t118621 - t111584 - t111585;
    let tv4rho3sigma8 = t123493 - t35547 + t116059 + t109148 + t116061 - t109149 - t116062 - t116063 + t111577 + t32686 + t35550 + t123495;
    (tv4rho3sigma8,)
}
