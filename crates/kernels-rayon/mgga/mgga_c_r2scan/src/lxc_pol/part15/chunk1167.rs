//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1167/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1167(t39979: f64, t10810: f64, t2184: f64, t7629: f64, t10894: f64, t7625: f64, t26314: f64, t37755: f64, t39841: f64, t39958: f64, t39963: f64, t39965: f64, t39968: f64, t39969: f64, t39972: f64, t39975: f64, t39977: f64) -> f64 {
    let t39980 = 0.10975748638225852664e-1_f64 * t39979;
    let t39982 = t2184 * t10810 * t7629;
    let t39983 = 0.46230515946956099004e0_f64 * t39982;
    let t39984 = t10894 * t7625;
    let t39985 = 0.54878743191129263322e-2_f64 * t39984;
    let t39987 = t37755 * t39841 * t26314;
    let t39989 = 0.93149212406257582491e-1_f64 * t39958 + t39963 + t39965 + t39968 + 0.14282990759302185291e-1_f64 * t39969 + 0.2600466522016280569e0_f64 * t39972 + 0.10401866088065122276e1_f64 * t39975 - 0.21341733463216935736e0_f64 * t39977 - t39980 - t39983 + t39985 + 0.13099107994629972538e-1_f64 * t39987;
    t39989
}
