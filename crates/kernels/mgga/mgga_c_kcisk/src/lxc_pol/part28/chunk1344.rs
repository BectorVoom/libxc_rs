//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1344/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1344<F: Float>(t12254: F, t2029: F, t10008: F, t10879: F, t9740: F, t10831: F, t1310: F, t786: F, t10463: F, t117193: F, t117195: F, t117203: F, t117207: F, t34680: F, t113307: F, t7724: F, t9406: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t118460 = t12254 * t2029;
    let t118466 = t9740 * t10879 * t10008;
    let t118473 = t1310 * t10831 * t786;
    let t118474 = t2029 * t10463;
    let t118493 = 0.15476481481481481481e-2 * t117193;
    let t118494 = 0.15476481481481481481e-2 * t117195;
    let t118495 = 0.23214722222222222222e-2 * t117203;
    let t118497 = 0.15476481481481481481e-2 * t117207;
    let t118621 = t34680 / 8.0;
    let t120905 = 2.0 * t113307;
    let t120906 = t7724 * t9406;
    (t118460, t118466, t118473, t118474, t118493, t118494, t118495, t118497, t118621, t120905, t120906)
}
