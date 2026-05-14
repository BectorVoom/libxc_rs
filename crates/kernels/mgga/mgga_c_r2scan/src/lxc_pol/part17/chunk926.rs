//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 926/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk926<F: Float>(t322: F, t12894: F, t333: F, t335: F, t337: F, t339: F, t341: F, t1020: F, t1135: F, t1137: F, t12890: F, t12892: F, t2956: F, t343: F, t3765: F, t12828: F, t10533: F, t11305: F, t11319: F, t12348: F, t12355: F, t12683: F, t12849: F, t12851: F, t12854: F, t12856: F, t12883: F, t330: F, t352: F, t3549: F, t3556: F, t3675: F, t855: F) -> (F, F, F, F, F, F, F) {
    let t323 = t322 <= 0.0;
    let t331 = t322 <= 0.25e1;
    let t332 = 0.25e1 < t322;
    let t12895 = t333 * t12894;
    let t12897 = t335 * t12894;
    let t12899 = t337 * t12894;
    let t12901 = t339 * t12894;
    let t12903 = t341 * t12894;
    let t12908 = 0.1550653405116e2 * t1135 * t2956 - 0.4355305902528e1 * t3765 * t1020 - 0.2177652951264e1 * t1137 * t2956 - 0.8704e0 * t12890 - 0.17408e1 * t12892 - 0.8704e0 * t12895 - 0.4607056813647e1 * t12897 + 0.122462410087e2 * t12899 - 0.957855118103e1 * t12901 + 0.3101306810232e1 * t12903 - 0.362942158544e0 * t343 * t12894 - 0.64e0 * t12894;
    let t12918 = piecewise3(t332, t12828, 0.0);
    let t12929 = piecewise5(t323, t12849 * t330 + 2.0 * t12851 * t330 + t12854 * t330 + t12856 * t330, t331, t12883 + t12908, -0.63e1 * t3556 * t12683 - 0.42e1 * t12348 * t3675 - 0.945e1 * t11305 * t12683 - 0.21e1 * t3549 * t10533 - 0.105e1 * t855 * t12918 * t352 - 0.315e1 * t12355 * t3675 - 0.1575e1 * t3556 * t10533 - 0.23625e1 * t11319 * t12683);
    (t12895, t12897, t12899, t12901, t12903, t12918, t12929)
}
