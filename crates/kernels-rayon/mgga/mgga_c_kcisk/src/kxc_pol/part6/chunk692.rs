//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 692/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk692(t1011: f64, t1015: f64, t1022: f64, t11: f64, t12408: f64, t12410: f64, t12414: f64, t12425: f64, t12436: f64, t12442: f64, t12446: f64, t12450: f64, t139: f64, t157: f64, t197: f64, t201: f64, t3125: f64, t3190: f64, t3200: f64, t3207: f64, t3217: f64, t972: f64) -> f64 {
    let t12453 = -0.74295e-1_f64 * t12408 * t12410 - 0.4953e-1_f64 * t3207 * t12414 - 0.15918666666666666666e0_f64 * t139 * t11 * t3125 - 0.79593333333333333331e-1_f64 * t139 * t201 * t12425 + 0.5306222222222222222e-1_f64 * t139 * t157 * t972 - 0.1857375e-1_f64 * t3190 * t1022 - 0.371475e-1_f64 * t197 * t12436 + 0.371475e-1_f64 * t1011 * t3217 - 0.8255e-2_f64 * t3200 * t12442 + 0.371475e-1_f64 * t3207 * t12446 - 0.38523333333333333333e-1_f64 * t1015 * t12450;
    t12453
}
