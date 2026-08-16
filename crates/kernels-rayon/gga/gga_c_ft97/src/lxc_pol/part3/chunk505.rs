//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 505/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk505(t713: f64, t992: f64, t2600: f64, t2599: f64, t766: f64, t2607: f64, t2606: f64, t2360: f64, t258: f64, t505: f64, t2486: f64, t255: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3875 = t992 * t713;
    let t3876 = t2600 * t3875;
    let t3877 = t2599 * t3876;
    let t3880 = t992 * t766;
    let t3881 = t2607 * t3880;
    let t3882 = t2606 * t3881;
    let t3885 = t258 * t2360;
    let t3886 = t992 * t505;
    let t3887 = t3885 * t3886;
    let t3888 = t2606 * t3887;
    let t3891 = t2486 * t255;
    (t3876, t3877, t3881, t3882, t3885, t3886, t3887, t3888, t3891)
}
