//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 984/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk984(t13879: f64, t1897: f64, t702: f64, t13941: f64, t2508: f64, t779: f64, t13945: f64, t681: f64, t13942: f64, t650: f64, t270: f64, t47420: f64, t738: f64) -> (f64, f64, f64, f64, f64) {
    let t47616 = 0.76905262301422242837e-2_f64 * t1897 * t13879 * t702;
    let t47619 = 0.76905262301422242837e-2_f64 * t2508 * t779 * t13941;
    let t47629 = 0.76905262301422242837e-2_f64 * t681 * t13945;
    let t47631 = 0.10254034973522965712e-1_f64 * t650 * t13942;
    let t47634 = 0.76905262301422242837e-2_f64 * t270 * t738 * t47420;
    (t47616, t47619, t47629, t47631, t47634)
}
