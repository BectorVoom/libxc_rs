//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2142/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2142<F: Float>(t29807: F, t342: F, t355: F, t99566: F, t19462: F, t1976: F, t3303: F, t4900: F, t1000: F, t100586: F, t1043: F, t106659: F, t1089: F, t1096: F, t1097: F, t1668: F, t19429: F, t25461: F, t25464: F, t25473: F, t25605: F, t25699: F, t27411: F, t27419: F, t27580: F, t29739: F, t29743: F, t29751: F, t29871: F, t29884: F, t29887: F, t29888: F, t4866: F, t6258: F, t6392: F, t7135: F, t7145: F, t7151: F, t7159: F, t7160: F, t7828: F, t93497: F, t93897: F, t94016: F, t94080: F) -> (F, F, F) {
    let t106701 = t342 * t29807;
    let t106719 = t355 * t99566;
    let t106727 = t19462 * t1976;
    let t106730 = t3303 * t4900;
    let t106738 = -F::cast_from(0.52041769129231196772e1_f64) * t7159 * t25464 * t29887 * t1096 + F::cast_from(0.17347256376410398924e1_f64) * t25605 * t27411 * t1668 * t1089 + F::cast_from(0.17347256376410398924e1_f64) * t25605 * t7828 * t4866 * t1089 + F::cast_from(0.8673628188205199462e0_f64) * t7159 * t7160 * t7135 * t6392 - F::cast_from(0.65854491829355115987e0_f64) * t106701 * t1097 + F::cast_from(0.52041769129231196772e1_f64) * t25699 * t7160 * t29871 * t1096 + F::cast_from(0.17347256376410398924e1_f64) * t27419 * t27580 - F::cast_from(0.34694512752820797848e1_f64) * t94080 * t29739 * t106659 + F::cast_from(0.8673628188205199462e0_f64) * t25461 * t29884 + F::cast_from(0.8673628188205199462e0_f64) * t7151 * t7145 * t7135 * t6258 - F::cast_from(0.34694512752820797848e1_f64) * t93497 * t29743 * t106719 - F::cast_from(0.26020884564615598386e1_f64) * t94016 * t29751 * t1043 * t1089 - F::cast_from(0.65854491829355115987e0_f64) * t106727 * t1000 - F::cast_from(0.17347256376410398924e1_f64) * t93897 * t29743 * t106730 + F::cast_from(0.17347256376410398924e1_f64) * t25473 * t29888 - F::cast_from(0.26341796731742046394e1_f64) * t100586 * t19429;
    (t106719, t106730, t106738)
}
