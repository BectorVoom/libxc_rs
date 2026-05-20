//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2141/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2141<F: Float>(t106497: F, t106543: F, t106588: F, t106636: F, t1032: F, t6343: F, t1982: F, t3303: F, t4894: F, t100681: F, t1089: F, t1096: F, t1668: F, t1695: F, t1976: F, t19855: F, t20219: F, t25464: F, t25605: F, t25699: F, t27426: F, t27427: F, t27543: F, t27609: F, t27656: F, t29731: F, t29743: F, t29760: F, t29817: F, t4758: F, t4772: F, t7102: F, t7144: F, t7145: F, t7159: F, t7160: F, t7162: F, t7818: F, t7821: F, t7833: F, t93497: F, t93498: F, t93904: F, t94085: F, t99709: F, t99886: F, t99915: F) -> (F, F, F, F) {
    let t106638 = t106497 + t106543 + t106588 + t106636;
    let t106655 = t6343 * t1032;
    let t106656 = t1982 * t106655;
    let t106659 = t3303 * t4894;
    let t106684 = F::cast_from(0.26341796731742046394e1_f64) * t100681 * t4758 - F::cast_from(0.8673628188205199462e0_f64) * t99886 * t7833 + F::cast_from(0.17347256376410398924e1_f64) * t7159 * t7160 * t27543 * t1695 + F::cast_from(0.65854491829355115987e0_f64) * t7102 * t20219 + F::cast_from(0.17347256376410398924e1_f64) * t27609 * t27427 - F::cast_from(0.26020884564615598386e1_f64) * t7159 * t25464 * t29731 * t1096 + F::cast_from(0.8673628188205199462e0_f64) * t106656 * t7162 + F::cast_from(0.34694512752820797848e1_f64) * t94085 * t29743 * t106659 + F::cast_from(0.17347256376410398924e1_f64) * t25605 * t27426 * t1668 * t1089 + F::cast_from(0.17347256376410398924e1_f64) * t93904 * t29760 - F::cast_from(0.8673628188205199462e0_f64) * t7144 * t7145 * t1976 * t19855 - F::cast_from(0.17347256376410398924e1_f64) * t99709 * t7818 - F::cast_from(0.52041769129231196772e1_f64) * t25699 * t7145 * t7821 * t4772 + F::cast_from(0.17347256376410398924e1_f64) * t99915 * t27656 - F::cast_from(0.34694512752820797848e1_f64) * t93497 * t29817 * t93498;
    (t106638, t106655, t106659, t106684)
}
