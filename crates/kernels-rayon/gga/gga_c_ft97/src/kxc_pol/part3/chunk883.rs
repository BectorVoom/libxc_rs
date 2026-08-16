//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 883/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk883(t1136: f64, t17712: f64, t3826: f64, t4914: f64, t2330: f64, t5058: f64, t1882: f64, t4923: f64, t4973: f64, t713: f64, t2354: f64, t446: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17713 = t17712 * t1136;
    let t17715 = t4914 * t3826;
    let t17718 = t2330 * t5058;
    let t17720 = t1882 * t4923;
    let t17722 = t4973 * t713;
    let t17723 = t2354 * t17722;
    let t17724 = t446 * t17723;
    (t17713, t17715, t17718, t17720, t17722, t17724)
}
