//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2754/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2754<F: Float>(t14933: F, t2482: F, t2668: F, t2719: F, t2710: F, t4371: F, t9732: F, t10886: F, t14833: F, t808: F, t10900: F, t124: F, t14791: F, t2394: F, t40625: F, t40630: F, t40638: F, t40639: F, t40643: F, t40645: F, t40654: F, t40662: F, t40669: F, t40679: F, t40681: F, t40686: F, t4362: F, t4366: F, t4457: F, t50151: F, t50418: F, t799: F, t800: F) -> F {
    let t50681 = t2482 * t2719 * t2668 * t14933;
    let t50703 = t2710 * t9732 * t4371;
    let t50706 = t10886 * t808 * t14833;
    let t50707 = F::cast_from(0.15246000842785598468e-3_f64) * t50706;
    let t50711 = -F::cast_from(0.8131200449485652516e-3_f64) * t50681 + F::cast_from(0.13553694749236397037e-5_f64) * t40625 + F::cast_from(0.1084295579938911763e-3_f64) * t40630 - t40638 + F::cast_from(0.86700792194318801432e-2_f64) * t40639 + F::cast_from(0.28582678745379824648e-4_f64) * t40643 - F::cast_from(0.91464571985215438874e-3_f64) * t40645 + t40654 - t799 * t800 * t124 * t50151 / F::new(48.0) - F::cast_from(0.51448821741683684367e-2_f64) * t4362 * t14791 * t50418 * t4366 - F::cast_from(0.30492001685571196935e-3_f64) * t40662 + F::cast_from(0.60023625365297631762e-1_f64) * t40669 - F::new(3.0) / F::new(4.0) * t10900 * t800 * t4457 * t2394 + F::cast_from(0.3252886739816735289e-3_f64) * t50703 - t50707 + F::cast_from(0.45732285992607719436e-3_f64) * t40679 - F::cast_from(0.81312004494856525156e-2_f64) * t40681 - F::cast_from(0.15246000842785598467e-2_f64) * t40686;
    t50711
}
