//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2244/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2244<F: Float>(t26948: F, t97065: F, t104606: F, t105021: F, t1203: F, t1214: F, t1294: F, t18090: F, t2149: F, t2150: F, t2151: F, t26906: F, t26937: F, t26969: F, t29109: F, t29122: F, t29158: F, t29179: F, t29217: F, t29264: F, t29282: F, t3552: F, t3575: F, t3601: F, t3738: F, t3783: F, t473: F, t5236: F, t5457: F, t7602: F, t7636: F, t7651: F, t7652: F, t8192: F, t8197: F, t8208: F, t8213: F, t96927: F, t97050: F, t97066: F, t97308: F, t97363: F, t97377: F, t97453: F) -> F {
    let t105046 = t26948 * t97065;
    let t105057 = -F::cast_from(0.8673628188205199462e0_f64) * t97453 * t8213 - F::cast_from(0.17347256376410398924e1_f64) * t97363 * t29217 - F::cast_from(0.8673628188205199462e0_f64) * t97308 * t104606 * t3783 + F::cast_from(0.65854491829355115987e0_f64) * t3552 * t8192 + F::cast_from(0.4336814094102599731e0_f64) * t26906 * t29122 * t3601 * t3783 + F::cast_from(0.10408353825846239354e2_f64) * t7651 * t97377 * t8208 * t3738 - F::cast_from(0.4336814094102599731e0_f64) * t2149 * t2150 * t473 * t105021 + F::cast_from(0.17347256376410398924e1_f64) * t7651 * t7652 * t29109 * t1294 + F::cast_from(0.34694512752820797848e1_f64) * t7636 * t7652 * t29282 * t1203 - F::cast_from(0.34694512752820797848e1_f64) * t96927 * t29158 * t5457 * t3575 - F::cast_from(0.65854491829355115987e0_f64) * t7602 * t18090 - F::cast_from(0.52041769129231196772e1_f64) * t97050 * t29264 - F::cast_from(0.69389025505641595696e1_f64) * t97066 * t2151 * t5236 * t1203 + F::cast_from(0.10408353825846239354e2_f64) * t105046 * t2151 * t5236 * t1214 - F::cast_from(0.52041769129231196772e1_f64) * t7636 * t26969 * t8197 * t3738 + F::cast_from(0.17347256376410398924e1_f64) * t26937 * t29179;
    t105057
}
