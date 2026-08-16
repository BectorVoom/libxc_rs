//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 899/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk899(t2508: f64, t2963: f64, t3276: f64, t3209: f64, t8483: f64, t7226: f64, t13209: f64, t7137: f64, t1841: f64, t8878: f64, t9748: f64, t10053: f64, t1024: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t43237 = 0.53833683610995569986e-1_f64 * t2508 * t3276 * t2963;
    let t43240 = t8483 * t3209;
    let t43243 = 0.46143157380853345701e-1_f64 * t2508 * t7226 * t43240;
    let t43254 = 0.10254034973522965712e-1_f64 * t7137 * t13209;
    let t43257 = 0.25635087433807414279e-2_f64 * t1841 * t8878 * t9748;
    let t43260 = 0.76905262301422242837e-2_f64 * t2508 * t10053 * t1024;
    (t43237, t43240, t43243, t43254, t43257, t43260)
}
