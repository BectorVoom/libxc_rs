//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1347/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1347<F: Float>(t19753: F, t32045: F, t5600: F, t32096: F, t33469: F, t32019: F, t110435: F, t2173: F, t3922: F, t6204: F, t32069: F, t3988: F, t20160: F, t33399: F, t9426: F, t33570: F, t3748: F) -> (F, F, F, F, F, F, F, F) {
    let t113615 = t5600 * t32045 * t19753;
    let t113620 = 0.23148148148148148148e-2 * t32096 * t33469;
    let t113622 = 0.23148148148148148148e-2 * t32019 * t33469;
    let t113629 = t6204 * t110435 * t2173 * t3922;
    let t113636 = t6204 * t32069 * t2173 * t3988;
    let t113639 = t20160 * t33399;
    let t113641 = 0.26805555555555555556e-2 * t9426 * t113639;
    let t113642 = t3748 * t33570;
    (t113615, t113620, t113622, t113629, t113636, t113639, t113641, t113642)
}
