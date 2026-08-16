//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1026/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1026(t12894: f64, t333: f64, t335: f64, t337: f64, t339: f64, t341: f64, t1020: f64, t1135: f64, t1137: f64, t12890: f64, t12892: f64, t2956: f64, t343: f64, t3765: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12895 = t333 * t12894;
    let t12897 = t335 * t12894;
    let t12899 = t337 * t12894;
    let t12901 = t339 * t12894;
    let t12903 = t341 * t12894;
    let t12908 = 0.1550653405116e2_f64 * t1135 * t2956 - 0.4355305902528e1_f64 * t3765 * t1020 - 0.2177652951264e1_f64 * t1137 * t2956 - 0.8704e0_f64 * t12890 - 0.17408e1_f64 * t12892 - 0.8704e0_f64 * t12895 - 0.4607056813647e1_f64 * t12897 + 0.122462410087e2_f64 * t12899 - 0.957855118103e1_f64 * t12901 + 0.3101306810232e1_f64 * t12903 - 0.362942158544e0_f64 * t343 * t12894 - 0.64e0_f64 * t12894;
    (t12895, t12897, t12899, t12901, t12903, t12908)
}
