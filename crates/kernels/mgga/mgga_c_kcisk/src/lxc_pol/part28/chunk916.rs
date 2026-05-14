//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 916/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk916<F: Float>(t17276: F, t1773: F, t4989: F, t7253: F, t1772: F, t7202: F, t25: F, t7269: F, t2448: F, t3934: F, t654: F, t10879: F, t2459: F, t7230: F, t10798: F, t7257: F) -> (F, F, F, F, F, F, F, F) {
    let t17277 = t1773 * t17276;
    let t17280 = 0.35981577432354634426e-1 * t4989 * t7253;
    let t17290 = t7202 * t1772;
    let t17293 = t25 * t7269;
    let t17295 = 0.35981577432354634426e-1 * t1773 * t17293;
    let t17317 = t2448 * t654 * t3934;
    let t17326 = t10879 * t2459;
    let t17327 = t1773 * t17326;
    let t17330 = 0.11993859144118211475e-1 * t4989 * t7230;
    let t17333 = t10798 * t7257;
    (t17277, t17280, t17290, t17295, t17317, t17327, t17330, t17333)
}
