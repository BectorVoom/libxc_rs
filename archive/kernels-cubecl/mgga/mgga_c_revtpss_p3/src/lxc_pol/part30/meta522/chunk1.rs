//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1934/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1934<F: Float>(t27960: F, t545: F, t2028: F, t1904: F, t2027: F, t2030: F, t26062: F, t26065: F, t26067: F, t26071: F, t26073: F, t26084: F, t27987: F, t27990: F, t27992: F, t28003: F, t28008: F, t5728: F, t7279: F, t7292: F, t7295: F, t7308: F, t7917: F, t7930: F) -> (F, F, F) {
    let t28011 = t545 * t27960;
    let t28012 = t2028 * t28011;
    let t28017 = -F::cast_from(0.54878743191129263322e-2_f64) * t27987 - F::cast_from(0.72280234901709995518e-2_f64) * t27990 + F::cast_from(0.12851425765524037203e-1_f64) * t27992 + F::cast_from(0.13170898365871023197e1_f64) * t7279 * t5728 - F::cast_from(0.65854491829355115987e0_f64) * t26084 * t1904 + F::cast_from(0.54878743191129263322e-2_f64) * t26062 + F::cast_from(0.9757440539382783019e-2_f64) * t26065 - F::cast_from(0.12851425765524037203e-1_f64) * t26067 - t26071 + F::cast_from(0.72280234901709995518e-2_f64) * t26073 + F::cast_from(0.8673628188205199462e0_f64) * t7295 * t28003 - F::cast_from(0.4336814094102599731e0_f64) * t7917 * t7308 - F::cast_from(0.4336814094102599731e0_f64) * t28008 * t2030 - F::cast_from(0.4336814094102599731e0_f64) * t2027 * t28012 - F::cast_from(0.4336814094102599731e0_f64) * t7292 * t7930;
    (t28011, t28012, t28017)
}
