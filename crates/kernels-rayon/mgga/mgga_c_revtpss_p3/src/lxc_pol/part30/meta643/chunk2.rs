//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2251/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2251(t1811: f64, t7642: f64, t8945: f64, t104480: f64, t2149: f64, t3596: f64, t104473: f64, t104483: f64, t105122: f64, t1214: f64, t17807: f64, t17855: f64, t17883: f64, t18047: f64, t2148: f64, t2152: f64, t26895: f64, t26897: f64, t26941: f64, t26979: f64, t26991: f64, t29111: f64, t29119: f64, t29158: f64, t29160: f64, t29199: f64, t29200: f64, t29201: f64, t29227: f64, t29275: f64, t29282: f64, t3739: f64, t5480: f64, t7602: f64, t7643: f64, t7648: f64, t7652: f64, t8217: f64, t97363: f64) -> f64 {
    let t105364 = t7642 * t1811;
    let t105365 = t105364 * t8945;
    let t105383 = t2149 * t104480 * t3596;
    let t105402 = -0.8673628188205199462e0_f64 * t7648 * t29111 - 0.17347256376410398924e1_f64 * t97363 * t29160 + 0.17347256376410398924e1_f64 * t105365 * t26897 + 0.8673628188205199462e0_f64 * t26895 * t29158 * t17883 - 0.4336814094102599731e0_f64 * t26991 * t8217 - 0.4336814094102599731e0_f64 * t2148 * t17807 * t2152 - 0.17347256376410398924e1_f64 * t29275 * t26941 + 0.17347256376410398924e1_f64 * t26979 * t29119 - 0.13170898365871023197e1_f64 * t7602 * t18047 + 0.26020884564615598386e1_f64 * t105383 * t104483 * t17855 + 0.8673628188205199462e0_f64 * t7648 * t29199 * t29201 + 0.8673628188205199462e0_f64 * t29200 * t105122 * t5480 + 0.8673628188205199462e0_f64 * t29200 * t104473 * t5480 + 0.13170898365871023197e1_f64 * t29227 * t3739 - 0.34694512752820797848e1_f64 * t7643 * t7652 * t29282 * t1214;
    t105402
}
