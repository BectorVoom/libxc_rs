//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 949/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk949(t21922: f64, t21937: f64, t1156: f64, t11285: f64, t21906: f64, t1137: f64, t21854: f64, t1671: f64, t18686: f64, t4740: f64, t6021: f64, t14850: f64, t6024: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t21938 = t21922 + t21937;
    let t21939 = t21938 * t1156;
    let t21942 = t21906 * t11285;
    let t21947 = t21906 * t1156;
    let t21952 = t21854 * t1137;
    let t21956 = 3.0_f64 * t18686 * t1671;
    let t21958 = 3.0_f64 * t4740 * t6021;
    let t21960 = 0.48245938496077605201e2_f64 * t14850 * t6024;
    (t21938, t21939, t21942, t21947, t21952, t21956, t21958, t21960)
}
