//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2952/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2952(t3088: f64, t53739: f64, t12167: f64, t1042: f64, t1063: f64, t11632: f64, t11653: f64, t11788: f64, t15689: f64, t15691: f64, t15700: f64, t15935: f64, t16104: f64, t16222: f64, t16226: f64, t19878: f64, t3105: f64, t3133: f64, t3151: f64, t3155: f64, t42155: f64, t42450: f64, t42454: f64, t4839: f64, t53450: f64, t53724: f64, t53728: f64, t53729: f64, t53735: f64, t606: f64, t905: f64) -> (f64, f64) {
    let t53740 = t3088 * t53739;
    let t53741 = t12167 * t53740;
    let t53759 = 0.25724410870841842183e-2_f64 * t1063 * t1042 * t15935 * t53450 + 0.85748036236139473944e-3_f64 * t19878 * t11653 + 0.50813651102897466041e-3_f64 * t53724 - 0.15879265969655458138e-3_f64 * t42450 - 0.95275595817932748827e-3_f64 * t42454 + 0.25724410870841842183e-2_f64 * t15700 * t53728 * t53729 - 0.71456696863449561621e-3_f64 * t15689 * t16222 * t53735 + 0.25724410870841842184e-2_f64 * t53741 * t15691 * t11632 * t3151 * t905 * t606 + 0.85748036236139473944e-3_f64 * t16226 * t15691 * t3155 * t3133 * t905 * t606 - 0.85748036236139473944e-3_f64 * t42155 * t16104 - 0.13719685797782315831e-1_f64 * t11788 * t3105 * t4839;
    (t53740, t53759)
}
