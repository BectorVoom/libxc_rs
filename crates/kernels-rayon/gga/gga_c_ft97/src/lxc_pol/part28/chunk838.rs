//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 838/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk838(t71: f64, t938: f64, t420: f64, t1301: f64, t32296: f64, t6427: f64, t52: f64, t7182: f64, t22736: f64, t22842: f64, t32133: f64, t32146: f64, t32185: f64, t32228: f64, t32247: f64, t32251: f64, t32259: f64, t32295: f64, t32301: f64, t34421: f64, t34424: f64, t34427: f64, t34430: f64, t6450: f64, t931: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34433 = t71 * t938;
    let t34434 = t420 * t34433;
    let t34435 = t1301 * t34434;
    let t34440 = t32296 * t6427;
    let t34444 = t52 * t7182 * t938;
    let t34450 = -0.23254900946437792e-1_f64 * t32301 * t931 - 2.0_f64 * t34421 + 0.10338048737805743097e-3_f64 * t32251 * t34424 + 0.88910709717637694816e-2_f64 * t22842 * t34427 + 0.89080607335887169333e-3_f64 * t32146 * t34430 + 0.15322466011111111111e0_f64 * t32259 * t34435 - t32185 - 0.25537443351851851852e-1_f64 * t32247 * t6450 - 0.25845121844514357744e-4_f64 * t32295 * t34440 - 0.22227677429409423704e-2_f64 * t32228 * t34444 - 0.22979081259345929704e-6_f64 * t22736 * t32133 * t6427;
    (t34433, t34434, t34435, t34440, t34444, t34450)
}
