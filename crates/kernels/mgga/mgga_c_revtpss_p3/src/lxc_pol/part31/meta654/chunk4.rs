//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2190/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2190<F: Float>(t27989: F, t98380: F, t689: F, t6919: F, t7242: F, t1904: F, t2022: F, t22386: F, t25924: F, t27868: F, t27980: F, t28008: F, t6895: F, t7274: F, t7295: F, t7296: F, t75188: F, t75267: F, t7930: F, t94409: F, t94580: F, t94591: F, t94593: F, t97719: F, t97734: F, t98056: F) -> F {
    let t108153 = t98380 * t27989;
    let t108156 = t689 * t7242 * t6919;
    let t108172 = t97719 - t94409 + F::cast_from(0.8673628188205199462e0_f64) * t7295 * t7296 * t2022 * t22386 - t97734 - F::cast_from(0.13170898365871023197e1_f64) * t98056 * t1904 + F::cast_from(0.65049603595885220126e-3_f64) * t94580 + F::cast_from(0.25702851531048074406e-1_f64) * t108153 + F::cast_from(0.54878743191129263322e-2_f64) * t108156 - F::cast_from(0.8673628188205199462e0_f64) * t28008 * t7930 - F::cast_from(0.8673628188205199462e0_f64) * t27868 * t27980 * t75267 + F::cast_from(0.45699670022203476294e-2_f64) * t94591 - F::cast_from(0.17347256376410398924e1_f64) * t27868 * t27980 * t75188 - F::cast_from(0.26020884564615598386e1_f64) * t7295 * t25924 * t7274 * t6895 + F::cast_from(0.17135234354032049604e-1_f64) * t94593;
    t108172
}
