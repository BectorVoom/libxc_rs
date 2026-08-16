//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1101/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1101(t2933: f64, t4719: f64, t1670: f64, t9752: f64, t2944: f64, t2960: f64, t4625: f64, t934: f64, t2952: f64, t4700: f64, t287: f64, t330: f64) -> (f64, f64, f64, f64, f64) {
    let t13878 = 2.0_f64 * t2933 * t4719;
    let t13880 = t9752 * t1670;
    let t13881 = t13880 * t2944;
    let t13885 = t2960 * t4625;
    let t13886 = t13885 * t934;
    let t13888 = t4700 * t2952;
    let t13890 = t287 * t330;
    (t13878, t13881, t13886, t13888, t13890)
}
