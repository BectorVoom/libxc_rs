//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 372/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk372(t2386: f64, t600: f64, t1678: f64, t1681: f64, t2366: f64, t2373: f64, t2376: f64, t2379: f64) -> (f64, f64) {
    let t2387 = t2386 * t600;
    let t2394 = 0.258925e1_f64 * t2373 - t1678 - 0.301925e0_f64 * t2366 + 0.16504875e0_f64 * t2376 - t1681 - 0.82785e-1_f64 * t2379;
    (t2387, t2394)
}
