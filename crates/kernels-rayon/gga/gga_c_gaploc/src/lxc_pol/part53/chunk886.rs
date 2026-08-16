//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 886/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk886(t42933: f64, t3247: f64, t32692: f64, t9647: f64, t10697: f64, t9624: f64, t2558: f64, t33348: f64, t1022: f64, t3209: f64) -> (f64, f64, f64, f64, f64) {
    let t42934 = 0.1922631557535556071e-2_f64 * t42933;
    let t42936 = t9647 * t32692 * t3247;
    let t42937 = 0.1922631557535556071e-2_f64 * t42936;
    let t42939 = t9647 * t10697 * t9624;
    let t42940 = 0.1922631557535556071e-2_f64 * t42939;
    let t42942 = t9647 * t33348 * t2558;
    let t42943 = 0.64087718584518535698e-3_f64 * t42942;
    let t42944 = t1022 * t3209;
    (t42934, t42937, t42940, t42943, t42944)
}
