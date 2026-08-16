//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2251/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2251<F: Float>(t1811: F, t7642: F, t8945: F, t104480: F, t2149: F, t3596: F, t104473: F, t104483: F, t105122: F, t1214: F, t17807: F, t17855: F, t17883: F, t18047: F, t2148: F, t2152: F, t26895: F, t26897: F, t26941: F, t26979: F, t26991: F, t29111: F, t29119: F, t29158: F, t29160: F, t29199: F, t29200: F, t29201: F, t29227: F, t29275: F, t29282: F, t3739: F, t5480: F, t7602: F, t7643: F, t7648: F, t7652: F, t8217: F, t97363: F) -> F {
    let t105364 = t7642 * t1811;
    let t105365 = t105364 * t8945;
    let t105383 = t2149 * t104480 * t3596;
    let t105402 = -F::cast_from(0.8673628188205199462e0_f64) * t7648 * t29111 - F::cast_from(0.17347256376410398924e1_f64) * t97363 * t29160 + F::cast_from(0.17347256376410398924e1_f64) * t105365 * t26897 + F::cast_from(0.8673628188205199462e0_f64) * t26895 * t29158 * t17883 - F::cast_from(0.4336814094102599731e0_f64) * t26991 * t8217 - F::cast_from(0.4336814094102599731e0_f64) * t2148 * t17807 * t2152 - F::cast_from(0.17347256376410398924e1_f64) * t29275 * t26941 + F::cast_from(0.17347256376410398924e1_f64) * t26979 * t29119 - F::cast_from(0.13170898365871023197e1_f64) * t7602 * t18047 + F::cast_from(0.26020884564615598386e1_f64) * t105383 * t104483 * t17855 + F::cast_from(0.8673628188205199462e0_f64) * t7648 * t29199 * t29201 + F::cast_from(0.8673628188205199462e0_f64) * t29200 * t105122 * t5480 + F::cast_from(0.8673628188205199462e0_f64) * t29200 * t104473 * t5480 + F::cast_from(0.13170898365871023197e1_f64) * t29227 * t3739 - F::cast_from(0.34694512752820797848e1_f64) * t7643 * t7652 * t29282 * t1214;
    t105402
}
