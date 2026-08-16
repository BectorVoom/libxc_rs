//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 872/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk872(t761: f64, t9919: f64, t2531: f64, t2535: f64, t2427: f64, t2430: f64, t185: f64, t9258: f64, t707: f64, t32: f64, t717: f64, t2659: f64) -> (f64, f64, f64, f64, f64) {
    let t9921 = 0.35089341735807877242e1_f64 * t761 * t9919;
    let t9922 = t2531 * t2535;
    let t9923 = 0.17544670867903938621e1_f64 * t9922;
    let t9924 = t2427 * t2430;
    let t9925 = 24.0_f64 * t9924;
    let t9926 = t185 * t9258;
    let t9928 = 4.0_f64 * t707 * t9926;
    let t9929 = t32 * t717;
    let t9931 = 36.0_f64 * t9929 * t2659;
    (t9921, t9923, t9925, t9928, t9931)
}
