//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 880/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk880(t34947: f64, t605: f64, t12664: f64, t7400: f64, t5935: f64, t6718: f64, t1053: f64, t32729: f64, t1384: f64, t26590: f64, t34918: f64, t525: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34948 = t605 * t34947;
    let t34950 = t12664 * t7400;
    let t34952 = t5935 * t6718;
    let t34954 = t32729 * t1053;
    let t34956 = t26590 * t1384;
    let t34961 = t525 * t34918;
    (t34948, t34950, t34952, t34954, t34956, t34961)
}
