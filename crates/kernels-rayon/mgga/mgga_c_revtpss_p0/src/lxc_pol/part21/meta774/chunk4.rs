//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2754/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2754(t14933: f64, t2482: f64, t2668: f64, t2719: f64, t2710: f64, t4371: f64, t9732: f64, t10886: f64, t14833: f64, t808: f64, t10900: f64, t124: f64, t14791: f64, t2394: f64, t40625: f64, t40630: f64, t40638: f64, t40639: f64, t40643: f64, t40645: f64, t40654: f64, t40662: f64, t40669: f64, t40679: f64, t40681: f64, t40686: f64, t4362: f64, t4366: f64, t4457: f64, t50151: f64, t50418: f64, t799: f64, t800: f64) -> f64 {
    let t50681 = t2482 * t2719 * t2668 * t14933;
    let t50703 = t2710 * t9732 * t4371;
    let t50706 = t10886 * t808 * t14833;
    let t50707 = 0.15246000842785598468e-3_f64 * t50706;
    let t50711 = -0.8131200449485652516e-3_f64 * t50681 + 0.13553694749236397037e-5_f64 * t40625 + 0.1084295579938911763e-3_f64 * t40630 - t40638 + 0.86700792194318801432e-2_f64 * t40639 + 0.28582678745379824648e-4_f64 * t40643 - 0.91464571985215438874e-3_f64 * t40645 + t40654 - t799 * t800 * t124 * t50151 / 48.0_f64 - 0.51448821741683684367e-2_f64 * t4362 * t14791 * t50418 * t4366 - 0.30492001685571196935e-3_f64 * t40662 + 0.60023625365297631762e-1_f64 * t40669 - 3.0_f64 / 4.0_f64 * t10900 * t800 * t4457 * t2394 + 0.3252886739816735289e-3_f64 * t50703 - t50707 + 0.45732285992607719436e-3_f64 * t40679 - 0.81312004494856525156e-2_f64 * t40681 - 0.15246000842785598467e-2_f64 * t40686;
    t50711
}
