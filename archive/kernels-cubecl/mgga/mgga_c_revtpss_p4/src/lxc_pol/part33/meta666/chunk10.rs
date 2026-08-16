//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2189/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2189<F: Float>(t1398: F, t543: F, t6895: F, t1904: F, t27985: F, t689: F, t108484: F, t108634: F, t2027: F, t2028: F, t25921: F, t25931: F, t26079: F, t26084: F, t30082: F, t4003: F, t545: F, t6919: F, t7295: F, t94823: F, t94914: F, t94917: F, t94919: F, t94931: F, t98382: F, t98384: F, t98387: F, t98390: F, t98399: F) -> F {
    let t108653 = t6895 * t1398 * t543;
    let t108662 = t689 * t27985 * t1904;
    let t108674 = t98382 + t98384 - t98387 + t98390 + F::cast_from(0.26020884564615598386e1_f64) * t94823 * t25931 * t108653 + F::cast_from(0.17135234354032049604e-2_f64) * t94914 - F::cast_from(0.65854491829355115987e0_f64) * t26084 * t6919 + t94917 - F::cast_from(0.24093411633903331839e-3_f64) * t94919 + t98399 - t94931 + F::cast_from(0.10975748638225852664e-1_f64) * t108662 - F::cast_from(0.8673628188205199462e0_f64) * t25921 * t30082 - F::cast_from(0.8673628188205199462e0_f64) * t7295 * t26079 * t108484 * t4003 - F::cast_from(0.4336814094102599731e0_f64) * t2027 * t2028 * t545 * t108634;
    t108674
}
