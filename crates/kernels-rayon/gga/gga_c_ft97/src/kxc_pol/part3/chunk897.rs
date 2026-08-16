//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 897/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk897(t17839: f64, t213: f64, t1109: f64, t679: f64, t689: f64, t1095: f64, t2382: f64, t2379: f64, t4939: f64, t807: f64, t236: f64, t688: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17840 = t17839 * t213;
    let t17841 = t1109 * t679;
    let t17842 = t17841 * t689;
    let t17843 = t17840 * t17842;
    let t17846 = t1095 * t2382;
    let t17847 = t2379 * t17846;
    let t17850 = t4939 * t2382;
    let t17851 = t2379 * t17850;
    let t17854 = t807 * t17850;
    let t17855 = t236 * t688;
    (t17843, t17847, t17850, t17851, t17854, t17855)
}
