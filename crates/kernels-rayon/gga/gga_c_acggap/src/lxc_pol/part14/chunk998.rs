//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 998/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk998(t35286: f64, t33953: f64, t4210: f64, t13364: f64, t31057: f64, t13299: f64, t31115: f64, t33938: f64, t7433: f64, t8779: f64, t1181: f64, t21955: f64, t30806: f64, t599: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35287 = 0.42874018118069736972e-3_f64 * t35286;
    let t35288 = t33953 * t4210;
    let t35290 = t31057 * t13364 * t35288;
    let t35291 = 0.21437009059034868486e-3_f64 * t35290;
    let t35301 = t31115 * t13299 * t33938;
    let t35302 = 0.15724046144802076034e-2_f64 * t35301;
    let t35307 = t7433 * t8779;
    let t35308 = 0.25724410870841842184e-2_f64 * t35307;
    let t35315 = t30806 * t1181 * t599 * t21955;
    (t35287, t35288, t35291, t35302, t35308, t35315)
}
