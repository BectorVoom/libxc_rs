//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 876/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk876(t10668: f64, t13498: f64, t13672: f64, t1897: f64, t2508: f64, t2580: f64, t2936: f64, t3451: f64, t43127: f64, t43139: f64, t44921: f64, t44924: f64, t44927: f64, t44928: f64, t44931: f64, t44933: f64, t44936: f64, t44938: f64, t44940: f64, t44956: f64, t44960: f64, t44963: f64, t702: f64, t7129: f64, t8637: f64) -> f64 {
    let t44964 = -t44921 + t44924 - t44927 + 0.12817543716903707139e-2_f64 * t44928 + t44931 - t44933 - t44936 + t44938 + 0.15381052460284448567e-1_f64 * t2508 * t2580 * t44940 - 0.76905262301422242837e-2_f64 * t1897 * t13672 * t702 - 0.46143157380853345702e-1_f64 * t7129 * t13498 - 0.46143157380853345702e-1_f64 * t2508 * t8637 * t3451 - 0.46143157380853345702e-1_f64 * t2508 * t2936 * t10668 - t44956 + 0.1281754371690370714e-2_f64 * t43127 + 0.17090058289204942853e-2_f64 * t43139 + t44960 + t44963;
    t44964
}
