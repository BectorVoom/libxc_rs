//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 733/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk733(t13871: f64, t396: f64, t12951: f64, t403: f64, t1390: f64, t301: f64, t1310: f64, t12829: f64, t1311: f64, t164: f64, t25: f64, t3951: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13873 = 0.19989765240197019125e-1_f64 * t396 * t13871;
    let t13878 = t403 * t12951;
    let t13893 = 1.0_f64 / t301 / t1390;
    let t13894 = t1310 * t13893;
    let t13895 = t403 * t12829;
    let t13900 = t164 * t1311;
    let t13917 = t25 * t3951;
    (t13873, t13878, t13894, t13895, t13900, t13917)
}
