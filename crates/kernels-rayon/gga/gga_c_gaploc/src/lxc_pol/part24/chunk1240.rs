//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1240/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1240(t10763: f64, t7129: f64, t2508: f64, t2717: f64, t2927: f64, t8979: f64, t954: f64, t21636: f64, t3448: f64, t21571: f64, t10714: f64, t10718: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t32529 = 0.46143157380853345702e-1_f64 * t7129 * t10763;
    let t32532 = 0.15381052460284448567e-1_f64 * t2508 * t2717 * t2927;
    let t32535 = 0.76905262301422242837e-2_f64 * t2508 * t954 * t8979;
    let t32539 = 0.6836023315681977141e-2_f64 * t21636 * t3448;
    let t32541 = 0.15381052460284448567e-1_f64 * t21571 * t3448;
    let t32543 = 0.30762104920568897134e-1_f64 * t7129 * t10714;
    let t32545 = 0.30762104920568897134e-1_f64 * t7129 * t10718;
    (t32529, t32532, t32535, t32539, t32541, t32543, t32545)
}
