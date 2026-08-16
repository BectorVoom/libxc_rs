//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1211/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1211(t1742: f64, t435: f64, t1111: f64, t384: f64, t398: f64, t1008: f64, t6211: f64, t1416: f64, t301: f64, t1137: f64, t5598: f64, t5632: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22099 = t1742 * t435;
    let t22102 = t384 * t398 * t22099 * t1111;
    let t22105 = t1008 * t6211;
    let t22107 = t1416 * t301;
    let t22112 = t1137 * t5598;
    let t22114 = t1137 * t5632;
    (t22099, t22102, t22105, t22107, t22112, t22114)
}
