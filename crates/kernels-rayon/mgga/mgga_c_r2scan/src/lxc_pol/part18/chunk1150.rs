//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1150/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1150(t1020: f64, t1087: f64, t1089: f64, t1091: f64, t11987: f64, t11989: f64, t12660: f64, t12662: f64, t12664: f64, t12666: f64, t12668: f64, t2410: f64, t2956: f64, t2958: f64, t3388: f64, t3402: f64, t3406: f64, t3664: f64, t3668: f64, t839: f64, t9707: f64) -> f64 {
    let t42677 = -0.3831420472412e2_f64 * t1087 * t9707 + 0.3101306810232e2_f64 * t11987 * t1020 + 0.3101306810232e2_f64 * t3664 * t2410 + 0.1550653405116e2_f64 * t3402 * t2956 + 0.1550653405116e2_f64 * t1089 * t9707 - 0.4355305902528e1_f64 * t11989 * t1020 - 0.4355305902528e1_f64 * t3668 * t2410 - 0.2177652951264e1_f64 * t3406 * t2956 - 0.2177652951264e1_f64 * t1091 * t9707 - 0.9214113627294e1_f64 * t12660 * t839 + 0.367387230261e2_f64 * t12662 * t839 - 0.3831420472412e2_f64 * t12664 * t839 + 0.1550653405116e2_f64 * t12666 * t839 - 0.2177652951264e1_f64 * t12668 * t839 + 0.734774460522e2_f64 * t3388 * t2958;
    t42677
}
