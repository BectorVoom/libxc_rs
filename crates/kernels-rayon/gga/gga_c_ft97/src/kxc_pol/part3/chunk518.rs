//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 518/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk518(t2857: f64, t3691: f64, t446: f64, t1091: f64, t824: f64, t2665: f64, t3700: f64, t835: f64, t18: f64, t792: f64, t3704: f64, t89: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4034 = t2857 * t3691;
    let t4035 = t446 * t4034;
    let t4037 = t1091 * t824;
    let t4038 = t2665 * t4037;
    let t4039 = t446 * t4038;
    let t4041 = t835 * t3700;
    let t4042 = t446 * t4041;
    let t4044 = t792 * t18;
    let t4046 = t89 * t3704 * t4044;
    (t4034, t4035, t4037, t4038, t4039, t4041, t4042, t4044, t4046)
}
