//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 1072/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk1072(t11987: f64, t28609: f64, t11399: f64, t7877: f64, t2554: f64, t11977: f64, t2153: f64, t334: f64, t3768: f64, t3696: f64, t3781: f64, t11533: f64, t761: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t33326 = t28609 * t11987;
    let t33328 = t11399 * t7877;
    let t33329 = t33328 * t2554;
    let t33330 = t11977 * t33329;
    let t33333 = t2153 * t3768 * t334;
    let t33336 = t2153 * t3696 * t3781;
    let t33338 = t761 * t11533;
    (t33326, t33328, t33330, t33333, t33336, t33338)
}
