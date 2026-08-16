//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 341/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk341(t2097: f64, t334: f64, t1205: f64, t1208: f64, t2077: f64, t2084: f64, t2087: f64, t2090: f64) -> (f64, f64) {
    let t2098 = t2097 * t334;
    let t2105 = 0.258925e1_f64 * t2084 - t1205 - 0.301925e0_f64 * t2077 + 0.16504875e0_f64 * t2087 - t1208 - 0.82785e-1_f64 * t2090;
    (t2098, t2105)
}
