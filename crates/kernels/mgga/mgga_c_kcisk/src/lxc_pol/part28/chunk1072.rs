//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1072/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1072<F: Float>(t1908: F, t24495: F, t7552: F, t7644: F, t11966: F, t9162: F, t2028: F, t15866: F, t15871: F, t1994: F, t22252: F, t22256: F, t22260: F, t22263: F, t22265: F, t22269: F, t22272: F, t22275: F, t22281: F, t22286: F, t22292: F, t22297: F, t22299: F, t22301: F, t22303: F, t7645: F, t7648: F) -> (F, F, F, F, F) {
    let t24496 = t1908 * t24495;
    let t24499 = t7552 * t7644;
    let t24511 = t9162 * t11966;
    let t24512 = t24511 * t2028;
    let t24523 = -0.17411041666666666666e-2 * t22252 + 0.11607361111111111111e-2 * t22256 + 0.386e0 * t1994 * t24499 + 0.12381185185185185185e-1 * t22260 - 0.23214722222222222222e-2 * t22263 + 0.11349419753086419753e-1 * t22265 - 0.15476481481481481481e-2 * t22269 - 0.17411041666666666666e-2 * t22272 - 0.386e0 * t7648 * t7645 + 0.11607361111111111111e-2 * t22275 - 0.41270617283950617283e-2 * t15866 - 0.386e0 * t1994 * t24512 + 0.23214722222222222221e-2 * t22281 + 0.69644166666666666664e-2 * t22286 - 0.30952962962962962962e-2 * t15871 + 0.69644166666666666666e-2 * t22292 + 0.92858888888888888888e-2 * t22297 - 0.23214722222222222222e-2 * t22299 - 0.23214722222222222221e-2 * t22301 - 0.23214722222222222222e-2 * t22303;
    (t24496, t24499, t24511, t24512, t24523)
}
