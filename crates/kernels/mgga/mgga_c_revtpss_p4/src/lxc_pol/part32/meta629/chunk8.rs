//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2027/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2027<F: Float>(t110275: F, t93281: F, t103432: F, t103435: F, t103437: F, t103441: F, t103444: F, t106172: F, t106302: F, t18313: F, t18785: F, t25391: F, t26547: F, t26550: F, t27199: F, t28411: F, t28426: F, t28439: F, t30381: F, t30410: F, t6049: F, t7067: F, t7070: F, t7403: F, t886: F, t93118: F) -> F {
    let t110572 = t93281 * t110275;
    let t110576 = -F::cast_from(0.65854491829355115987e0_f64) * t7403 * t18785 - F::cast_from(0.17347256376410398924e1_f64) * t25391 * t26550 * t106302 - F::cast_from(0.4336814094102599731e0_f64) * t7067 * t30381 + F::cast_from(0.10408353825846239354e2_f64) * t7070 * t93118 * t30410 * t886 + F::cast_from(0.13170898365871023197e1_f64) * t26547 * t6049 - F::cast_from(0.68540937416128198416e-1_f64) * t103432 + t103435 - t103437 - t103441 + t103444 - F::cast_from(0.17347256376410398924e1_f64) * t106172 * t28426 + F::cast_from(0.8673628188205199462e0_f64) * t106172 * t28439 + F::cast_from(0.26341796731742046394e1_f64) * t7403 * t18313 + F::cast_from(0.43368140941025997311e-1_f64) * t110572 - F::cast_from(0.52041769129231196772e1_f64) * t27199 * t28411;
    t110576
}
