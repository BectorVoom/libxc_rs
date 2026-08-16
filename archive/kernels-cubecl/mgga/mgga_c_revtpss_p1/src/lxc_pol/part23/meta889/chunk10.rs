//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2829/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2829<F: Float>(t14791: F, t2745: F, t40409: F, t50370: F, t50372: F, t50375: F, t50377: F, t50381: F, t50383: F, t50385: F, t50387: F, t50390: F, t6035: F, t61572: F, t61574: F, t61576: F, t61582: F, t61612: F, t61616: F, t61749: F, t76302: F, t837: F) -> F {
    let t76458 = F::cast_from(0.4065600224742826258e-3_f64) * t61572 + F::cast_from(0.30011812682648815881e-2_f64) * t61574 + F::cast_from(0.4065600224742826258e-3_f64) * t61576 - F::cast_from(0.85748036236139473944e-4_f64) * t61582 - F::cast_from(0.20082057720118594944e-6_f64) * t40409 + F::cast_from(0.45351183609335988442e0_f64) * t50370 + F::cast_from(0.21675198048579700358e-2_f64) * t50372 - t50375 - F::cast_from(0.24098469264142313933e-5_f64) * t50377 + F::cast_from(0.33884236873090992593e-6_f64) * t50381 - F::cast_from(0.68026775414003982663e-1_f64) * t50383 - F::cast_from(0.15415400852149882895e-1_f64) * t50385 + F::cast_from(0.45732285992607719436e-2_f64) * t50387 + t50390 + F::cast_from(0.25724410870841842183e-2_f64) * t2745 * t14791 * t61749 * t6035 + F::cast_from(0.25724410870841842183e-2_f64) * t2745 * t14791 * t76302 * t837 - F::cast_from(0.17149607247227894789e-3_f64) * t61612 - F::cast_from(0.17149607247227894789e-3_f64) * t61616;
    t76458
}
