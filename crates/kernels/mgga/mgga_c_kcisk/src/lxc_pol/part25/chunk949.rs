//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 949/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk949<F: Float>(t3521: F, t7013: F, t1849: F, t2372: F, t3290: F, t4604: F, t16017: F, t7012: F, t11259: F, t2364: F, t4658: F, t11269: F, t4624: F, t16004: F, t7000: F, t1814: F, t4629: F) -> (F, F, F, F, F, F, F) {
    let t16784 = 0.17521145777777777778e-2 * t3521 * t7013;
    let t16787 = t4604 * t2372 * t1849 * t3290;
    let t16790 = t7012 * t16017;
    let t16794 = t11259 * t2364 * t4658;
    let t16798 = t11269 * t2364 * t4624;
    let t16801 = t7000 * t16004;
    let t16804 = t4629 * t1814;
    (t16784, t16787, t16790, t16794, t16798, t16801, t16804)
}
