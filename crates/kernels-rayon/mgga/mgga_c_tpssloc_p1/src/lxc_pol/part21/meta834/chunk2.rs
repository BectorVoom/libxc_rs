//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2955/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2955(t16558: f64, t2989: f64, t10224: f64, t5828: f64, t973: f64, t42875: f64, t5817: f64, t17763: f64, t2960: f64, t10241: f64, t10245: f64, t17794: f64, t17800: f64, t2986: f64, t2988: f64, t3014: f64, t343: f64, t4546: f64, t48397: f64, t48402: f64, t48407: f64, t48417: f64, t48421: f64, t5842: f64) -> f64 {
    let t61589 = t2989 * t16558;
    let t61597 = t973 * t10224 * t5828;
    let t61600 = t973 * t42875 * t5817;
    let t61602 = t2960 * t17763;
    let t61614 = -0.27777777777777777777e-3_f64 * t2986 * t10241 * t17794 - 0.55555555555555555554e-3_f64 * t2986 * t2988 * t61589 - 0.27777777777777777777e-3_f64 * t2986 * t17800 * t10245 - 0.6172839506172839506e-4_f64 * t61597 - 0.82304526748971193413e-4_f64 * t61600 + 0.98765432098765432095e-3_f64 * t61602 - 0.83333333333333333332e-3_f64 * t973 * t4546 * t5842 * t3014 * t343 + 0.20576131687242798353e-3_f64 * t48397 - 0.11111111111111111111e-2_f64 * t48402 - 0.55555555555555555554e-3_f64 * t48407 - 0.55555555555555555554e-3_f64 * t48417 + 0.18106995884773662551e-2_f64 * t48421;
    t61614
}
