//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 950/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk950(t3209: f64, t8483: f64, t2508: f64, t7226: f64, t13179: f64, t7129: f64, t10667: f64, t795: f64, t948: f64, t13209: f64, t7137: f64, t1841: f64, t8878: f64, t9748: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t43240 = t8483 * t3209;
    let t43243 = 0.46143157380853345701e-1_f64 * t2508 * t7226 * t43240;
    let t43244 = t7129 * t13179;
    let t43246 = t795 * t10667;
    let t43248 = t2508 * t43246 * t948;
    let t43254 = 0.10254034973522965712e-1_f64 * t7137 * t13209;
    let t43257 = 0.25635087433807414279e-2_f64 * t1841 * t8878 * t9748;
    (t43240, t43243, t43244, t43246, t43248, t43254, t43257)
}
