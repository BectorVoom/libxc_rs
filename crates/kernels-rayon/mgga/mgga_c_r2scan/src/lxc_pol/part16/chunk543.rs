//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 543/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk543(t2956: f64, t333: f64, t2958: f64, t335: f64, t337: f64, t339: f64, t341: f64, t343: f64, t1035: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2960 = t333 * t2956;
    let t2962 = t333 * t2958;
    let t2964 = t335 * t2956;
    let t2966 = t335 * t2958;
    let t2968 = t337 * t2956;
    let t2970 = t337 * t2958;
    let t2972 = t339 * t2956;
    let t2974 = t339 * t2958;
    let t2976 = t341 * t2956;
    let t2982 = -0.64e0_f64 * t2956 - 0.8704e0_f64 * t2958 - 0.8704e0_f64 * t2960 - 0.9214113627294e1_f64 * t2962 - 0.4607056813647e1_f64 * t2964 + 0.367387230261e2_f64 * t2966 + 0.122462410087e2_f64 * t2968 - 0.3831420472412e2_f64 * t2970 - 0.957855118103e1_f64 * t2972 + 0.1550653405116e2_f64 * t2974 + 0.3101306810232e1_f64 * t2976 - 0.2177652951264e1_f64 * t341 * t2958 - 0.362942158544e0_f64 * t343 * t2956;
    let t2983 = t1035 * t1035;
    (t2960, t2962, t2964, t2966, t2968, t2970, t2972, t2974, t2976, t2982, t2983)
}
