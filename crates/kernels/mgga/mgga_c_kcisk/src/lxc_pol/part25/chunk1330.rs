//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1330/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1330<F: Float>(t11694: F, t34306: F, t17772: F, t9718: F, t116067: F, t116069: F, t116072: F, t116074: F, t116077: F, t116080: F, t116083: F, t116085: F, t116087: F, t116089: F, t116091: F, t116094: F, t116096: F, t116098: F, t116101: F, t117258: F) -> (F, F, F) {
    let t117260 = 4.0 * t11694 * t34306;
    let t117262 = 2.0 * t17772 * t9718;
    let t117263 = t116067 + t116069 + t116072 - t116074 + t116077 - t116080 - t116083 - t116085 + t116087 + t116089 + t116091 + t116094 + t116096 - t116098 + t116101 + t117258 + t117260 - t117262;
    (t117260, t117262, t117263)
}
