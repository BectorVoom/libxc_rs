//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1021/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1021(t1018: f64, t3643: f64, t1079: f64, t2951: f64, t2953: f64, t1081: f64, t2958: f64, t2956: f64, t1020: f64, t1083: f64, t1085: f64, t1087: f64, t1089: f64, t1091: f64, t3652: f64, t3656: f64, t3660: f64, t3664: f64, t3668: f64) -> (f64, f64, f64, f64) {
    let t12624 = t3643 * t1018;
    let t12627 = t1079 * t2951;
    let t12629 = t1079 * t2953;
    let t12632 = t2958 * t1081;
    let t12654 = t2956 * t1081;
    let t12656 = -0.9214113627294e1_f64 * t12632 - 0.18428227254588e2_f64 * t3652 * t1020 - 0.9214113627294e1_f64 * t1083 * t2956 + 0.734774460522e2_f64 * t3656 * t1020 + 0.367387230261e2_f64 * t1085 * t2956 - 0.7662840944824e2_f64 * t3660 * t1020 - 0.3831420472412e2_f64 * t1087 * t2956 + 0.3101306810232e2_f64 * t3664 * t1020 + 0.1550653405116e2_f64 * t1089 * t2956 - 0.4355305902528e1_f64 * t3668 * t1020 - 0.2177652951264e1_f64 * t1091 * t2956 - 0.8704e0_f64 * t12654;
    (t12624, t12627, t12629, t12656)
}
