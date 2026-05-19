//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1339/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1339<F: Float>(t1882: F, t543: F, t6895: F, t2022: F, t22857: F, t108411: F, t108422: F, t108431: F, t114590: F, t213: F, t225: F, t23043: F, t25931: F, t26079: F, t27837: F, t27868: F, t27980: F, t30017: F, t30106: F, t4003: F, t561: F, t7279: F, t7295: F, t7301: F, t86413: F, t94683: F, t94823: F, t94854: F, t97933: F, t98011: F, t98029: F, t9994: F) -> F {
    let t114666 = t6895 * t1882 * t543;
    let t114671 = t2022 * t22857;
    let t114701 = F::cast_from(0.78062653693846795158e1_f64) * t94823 * t25931 * t114666 + F::cast_from(0.51405703062096148812e-1_f64) * t98011 + F::cast_from(0.26020884564615598386e1_f64) * t7295 * t94683 * t114671 * t9994 - F::cast_from(0.26020884564615598386e1_f64) * t7295 * t26079 * t114671 * t4003 - F::cast_from(0.65854491829355115987e0_f64) * t7279 * t23043 - F::cast_from(0.32927245914677557992e-1_f64) * t108411 + F::cast_from(0.57824187921367996415e-1_f64) * t98029 + F::cast_from(0.4336814094102599731e0_f64) * t7295 * t7301 * t114671 * t543 - F::cast_from(0.52041769129231196772e1_f64) * t97933 * t30106 - F::cast_from(0.26020884564615598386e1_f64) * t27868 * t27980 * t86413 - F::cast_from(0.29272321618148349057e-1_f64) * t108422 + F::cast_from(0.65854491829355115987e0_f64) * t213 * t114590 * t225 * t561 + t94854 + F::cast_from(0.43368140941025997312e-1_f64) * t108431 - F::cast_from(0.78062653693846795158e1_f64) * t27837 * t30017;
    t114701
}
