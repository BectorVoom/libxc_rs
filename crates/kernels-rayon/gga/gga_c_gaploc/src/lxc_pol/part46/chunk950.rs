//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 950/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk950(t10053: f64, t1024: f64, t2508: f64, t2927: f64, t3270: f64, t13221: f64, t7137: f64, t13217: f64, t13185: f64, t42920: f64, t723: f64, t40877: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t43260 = 0.76905262301422242837e-2_f64 * t2508 * t10053 * t1024;
    let t43263 = 0.76905262301422242837e-2_f64 * t2508 * t3270 * t2927;
    let t43265 = 0.10254034973522965712e-1_f64 * t7137 * t13221;
    let t43267 = 0.61524209841137794268e-1_f64 * t7137 * t13217;
    let t43269 = 0.71778244814660759981e-1_f64 * t7137 * t13185;
    let t43270 = t42920 * t723;
    let t43274 = 0.85450291446024714264e-3_f64 * t40877;
    (t43260, t43263, t43265, t43267, t43269, t43270, t43274)
}
