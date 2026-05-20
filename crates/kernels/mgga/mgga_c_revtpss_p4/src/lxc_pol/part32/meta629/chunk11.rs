//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2030/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2030<F: Float>(t18805: F, t95936: F, t103037: F, t103424: F, t106143: F, t106360: F, t106365: F, t110493: F, t14495: F, t231: F, t25391: F, t26547: F, t26550: F, t27199: F, t27353: F, t28405: F, t6072: F, t7070: F, t7076: F, t93349: F, t95911: F, t95914: F, t95925: F, t95927: F, t95930: F) -> F {
    let t110639 = t95936 * t18805;
    let t110665 = -F::cast_from(0.65854491829355115987e0_f64) * t26547 * t6072 + F::cast_from(0.19514881078765566037e-1_f64) * t110639 + F::cast_from(0.48186823267806663678e-3_f64) * t95911 + t95914 - F::cast_from(0.17347256376410398924e1_f64) * t25391 * t103037 * t14495 + F::cast_from(0.65049603595885220126e-3_f64) * t95925 - F::cast_from(0.13009920719177044025e-1_f64) * t95927 - t95930 - F::cast_from(0.8673628188205199462e0_f64) * t25391 * t26550 * t106143 + F::cast_from(0.26020884564615598386e1_f64) * t93349 * t26550 * t106360 + F::cast_from(0.8673628188205199462e0_f64) * t27353 * t103424 * t14495 - F::cast_from(0.8673628188205199462e0_f64) * t25391 * t26550 * t106365 + F::cast_from(0.8673628188205199462e0_f64) * t27199 * t28405 + F::cast_from(0.4336814094102599731e0_f64) * t7070 * t7076 * t110493 * t231;
    t110665
}
