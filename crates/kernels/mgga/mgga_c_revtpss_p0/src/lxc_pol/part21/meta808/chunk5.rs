//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2952/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2952<F: Float>(t3088: F, t53739: F, t12167: F, t1042: F, t1063: F, t11632: F, t11653: F, t11788: F, t15689: F, t15691: F, t15700: F, t15935: F, t16104: F, t16222: F, t16226: F, t19878: F, t3105: F, t3133: F, t3151: F, t3155: F, t42155: F, t42450: F, t42454: F, t4839: F, t53450: F, t53724: F, t53728: F, t53729: F, t53735: F, t606: F, t905: F) -> (F, F) {
    let t53740 = t3088 * t53739;
    let t53741 = t12167 * t53740;
    let t53759 = F::cast_from(0.25724410870841842183e-2_f64) * t1063 * t1042 * t15935 * t53450 + F::cast_from(0.85748036236139473944e-3_f64) * t19878 * t11653 + F::cast_from(0.50813651102897466041e-3_f64) * t53724 - F::cast_from(0.15879265969655458138e-3_f64) * t42450 - F::cast_from(0.95275595817932748827e-3_f64) * t42454 + F::cast_from(0.25724410870841842183e-2_f64) * t15700 * t53728 * t53729 - F::cast_from(0.71456696863449561621e-3_f64) * t15689 * t16222 * t53735 + F::cast_from(0.25724410870841842184e-2_f64) * t53741 * t15691 * t11632 * t3151 * t905 * t606 + F::cast_from(0.85748036236139473944e-3_f64) * t16226 * t15691 * t3155 * t3133 * t905 * t606 - F::cast_from(0.85748036236139473944e-3_f64) * t42155 * t16104 - F::cast_from(0.13719685797782315831e-1_f64) * t11788 * t3105 * t4839;
    (t53740, t53759)
}
