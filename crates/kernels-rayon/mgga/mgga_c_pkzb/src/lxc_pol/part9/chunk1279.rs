//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1279/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1279(t2380: f64, t6475: f64, t8345: f64, t1220: f64, t6377: f64, t3235: f64, t3237: f64, t5939: f64, t179: f64, t3026: f64, t404: f64, t6380: f64) -> (f64, f64, f64, f64) {
    let t22452 = t2380 * t6475 * t8345;
    let t22461 = t1220 * t6377;
    let t22469 = t3235 * t5939 * t3237;
    let t22474 = t404 * t179 * t6380 * t3026;
    (t22452, t22461, t22469, t22474)
}
