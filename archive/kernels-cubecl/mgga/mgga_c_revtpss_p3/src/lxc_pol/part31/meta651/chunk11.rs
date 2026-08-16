//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2163/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2163<F: Float>(t106823: F, t3153: F, t106659: F, t106745: F, t106824: F, t1089: F, t1096: F, t1651: F, t20172: F, t25464: F, t25605: F, t25611: F, t25629: F, t27415: F, t27418: F, t27543: F, t27640: F, t27664: F, t27669: F, t27676: F, t29727: F, t29744: F, t29759: F, t29876: F, t3046: F, t4758: F, t4866: F, t4976: F, t4983: F, t4998: F, t6350: F, t7135: F, t7140: F, t7145: F, t7151: F, t7159: F, t7160: F, t7817: F, t7818: F, t7825: F, t93497: F, t93963: F, t93983: F, t99721: F) -> F {
    let t107318 = t106823 * t3153;
    let t107354 = -F::cast_from(0.26020884564615598386e1_f64) * t7159 * t25464 * t7135 * t6350 - F::cast_from(0.8673628188205199462e0_f64) * t7825 * t27676 - F::cast_from(0.8673628188205199462e0_f64) * t27415 * t29876 - F::cast_from(0.34694512752820797848e1_f64) * t7151 * t7160 * t29727 * t1096 - F::cast_from(0.17347256376410398924e1_f64) * t27669 * t107318 * t4983 + F::cast_from(0.8673628188205199462e0_f64) * t27640 * t107318 * t4998 - F::cast_from(0.34694512752820797848e1_f64) * t93497 * t29759 * t106745 + F::cast_from(0.26341796731742046394e1_f64) * t99721 * t4758 + F::cast_from(0.34694512752820797848e1_f64) * t93983 * t29759 * t106659 + F::cast_from(0.13170898365871023197e1_f64) * t7140 * t20172 + F::cast_from(0.17347256376410398924e1_f64) * t7151 * t7145 * t27543 * t1651 - F::cast_from(0.17347256376410398924e1_f64) * t3046 * t27418 * t7818 + F::cast_from(0.17347256376410398924e1_f64) * t25611 * t106824 * t4976 + F::cast_from(0.17347256376410398924e1_f64) * t25605 * t106824 * t27664 - F::cast_from(0.17347256376410398924e1_f64) * t25629 * t7817 * t4866 * t1089 + F::cast_from(0.17347256376410398924e1_f64) * t93963 * t29744;
    t107354
}
