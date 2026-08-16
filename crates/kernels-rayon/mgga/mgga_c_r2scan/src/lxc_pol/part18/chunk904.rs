//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 904/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk904(t339: f64, t9707: f64, t341: f64, t1026: f64, t1028: f64, t1030: f64, t2410: f64, t2966: f64, t2968: f64, t2970: f64, t2972: f64, t2974: f64, t2976: f64, t343: f64, t839: f64) -> f64 {
    let t9738 = t339 * t9707;
    let t9746 = t341 * t9707;
    let t9756 = -0.11494261417236e3_f64 * t2966 * t839 - 0.7662840944824e2_f64 * t1026 * t2410 - 0.3831420472412e2_f64 * t2968 * t839 - 0.957855118103e1_f64 * t9738 + 0.6202613620464e2_f64 * t2970 * t839 + 0.3101306810232e2_f64 * t1028 * t2410 + 0.1550653405116e2_f64 * t2972 * t839 + 0.3101306810232e1_f64 * t9746 - 0.1088826475632e2_f64 * t2974 * t839 - 0.4355305902528e1_f64 * t1030 * t2410 - 0.2177652951264e1_f64 * t2976 * t839 - 0.362942158544e0_f64 * t343 * t9707;
    t9756
}
