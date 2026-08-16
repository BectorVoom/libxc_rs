//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 711/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk711<F: Float>(t1445: F, t2027: F, t2103: F, t213: F, t561: F, t7292: F, t7295: F, t7495: F, t7498: F, t7507: F, t7511: F, t7517: F, t7519: F, t7523: F, t7528: F, t7532: F) -> F {
    let t7535 = -t7495 + t7498 + F::cast_from(0.65854491829355115987e0_f64) * t213 * t7507 * t561 - F::cast_from(0.65854491829355115987e0_f64) * t7511 * t1445 + t7517 - t7519 - F::cast_from(0.4336814094102599731e0_f64) * t7292 * t2103 + F::cast_from(0.8673628188205199462e0_f64) * t7295 * t7523 + F::cast_from(0.4336814094102599731e0_f64) * t7295 * t7528 - F::cast_from(0.4336814094102599731e0_f64) * t2027 * t7532;
    t7535
}
