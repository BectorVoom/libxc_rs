//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 506/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk506<F: Float>(t1871: F, t2507: F, t1333: F, t2534: F, t2510: F, t2514: F, t3521: F, t4595: F, t708: F, t1876: F, t1417: F, t2518: F, t1646: F, t673: F, t2372: F, t682: F, sigma2: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t6973 = t2507 * t1871;
    let t6974 = t6973 * sigma2;
    let t6990 = t1333 * t2534;
    let t6992 = t1333 * t2510;
    let t6998 = t3521 * t2514;
    let t7000 = t4595 * t708;
    let t7012 = t1876 * t708;
    let t7020 = t1417 * t2518;
    let t7028 = t673 * t1646;
    let t7029 = t708 * t2372;
    let t7034 = t682 * t2372;
    (t6973, t6974, t6990, t6992, t6998, t7000, t7012, t7020, t7028, t7029, t7034)
}
