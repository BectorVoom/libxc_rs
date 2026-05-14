//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1299/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1299<F: Float>(t33994: F, t109134: F, t109135: F, t109136: F, t109141: F, t109144: F, t109148: F, t113271: F, t113272: F, t113273: F, t116027: F, t116028: F, t116029: F, t116031: F, t35048: F, t35055: F, t35059: F, t35062: F) -> (F, F) {
    let t116039 = t33994 / 8.0;
    let t118623 = t113271 - t113272 + t113273 - t35055 + t35059 - t116027 - t109134 + t109135 - t109136 - t35048 + t116028 + t109141 + t116029 + t109144 - t116031 - t35062 + t109148;
    (t116039, t118623)
}
