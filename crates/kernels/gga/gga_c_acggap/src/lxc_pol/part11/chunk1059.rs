//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1059/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1059<F: Float>(t157: F, t309: F, t463: F, t32130: F, t36433: F, t32029: F, t557: F, t1658: F, t406: F, t2934: F, t609: F, t2132: F, t2331: F, t7885: F, t864: F, t1659: F, t20138: F, t2127: F, t2155: F, t2338: F, t32196: F, t32201: F, t32210: F, t32219: F, t33566: F, t35324: F, t5340: F, t7879: F, t7931: F, t7932: F, t7934: F, t8001: F, t8400: F, t9033: F) -> (F,) {
    let t36495 = t157 * t463 * t309;
    let t36498 = 0.34694512752820797848e1 * t32130 * t36433 * t36495;
    let t36504 = t32029 * t557;
    let t36511 = t1658 * t406 * t157;
    let t36515 = t2934 * t609;
    let t36526 = t7885 * t2132 * t2331 * t864;
    let t36528 = -0.17347256376410398924e1 * t8400 * t9033 * t20138 - t36498 - 0.17347256376410398924e1 * t7931 * t36433 * t7934 + 0.17347256376410398924e1 * t32196 + 0.8673628188205199462e0 * t32201 - 0.65854491829355115987e0 * t36504 - 0.4336814094102599731e0 * t2338 * t7879 + 0.8673628188205199462e0 * t33566 * t2155 - t32210 - 0.17347256376410398924e1 * t7931 * t7932 * t36511 + 0.26020884564615598386e1 * t8400 * t36515 * t35324 + 0.13170898365871023197e1 * t2127 * t5340 + 0.17347256376410398924e1 * t32219 - 0.13170898365871023197e1 * t8001 * t1659 - 0.26020884564615598386e1 * t36526;
    (t36528,)
}
