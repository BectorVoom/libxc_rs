//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 778/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk778(t10791: f64, t1248: f64, t2364: f64, t2404: f64, t4857: f64, t4908: f64, t2541: f64, t5217: f64, t17056: f64, t740: f64, t5320: f64, t6973: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17385 = t1248 * t10791 * t2364;
    let t17520 = t2404 * t4857;
    let t17567 = t2404 * t4908;
    let t17775 = t2541 * t5217;
    let t17821 = t17056 * t740;
    let t17933 = t6973 * t5320;
    (t17385, t17520, t17567, t17775, t17821, t17933)
}
