//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 972/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk972<F: Float>(t28331: F, t28339: F, t225: F, t7997: F, t886: F, t7071: F, t27216: F, t7407: F, t213: F, t25383: F, t257: F, t26437: F, t26439: F, t26448: F, t26483: F, t26486: F, t28310: F, t28315: F, t28317: F, t4534: F, t7070: F, t7403: F, t7424: F, t7766: F, t8007: F) -> (F, F) {
    let t28340 = t28331 + t28339;
    let t28341 = t28340 * t225;
    let t28347 = t7997 * t886;
    let t28348 = t7071 * t28347;
    let t28352 = t27216 * t7407;
    let t28358 = -t26437 + t26439 - F::cast_from(0.54878743191129263322e-2_f64) * t26448 + F::cast_from(0.8673628188205199462e0_f64) * t7070 * t28310 - F::cast_from(0.14456046980341999104e-1_f64) * t28315 + F::cast_from(0.25702851531048074406e-1_f64) * t28317 + F::cast_from(0.12851425765524037203e-1_f64) * t26483 + F::cast_from(0.65854491829355115987e0_f64) * t213 * t28341 * t257 + F::cast_from(0.8673628188205199462e0_f64) * t25383 * t8007 + F::cast_from(0.8673628188205199462e0_f64) * t7070 * t28348 + F::cast_from(0.25702851531048074406e-1_f64) * t26486 - F::cast_from(0.12851425765524037203e-1_f64) * t28352 - F::cast_from(0.4336814094102599731e0_f64) * t7766 * t7424 - F::cast_from(0.65854491829355115987e0_f64) * t7403 * t4534;
    (t28340, t28358)
}
