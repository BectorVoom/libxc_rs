//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1041/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1041(t22668: f64, t6742: f64, t6745: f64, t22531: f64, t580: f64, t587: f64, t601: f64, t1963: f64, t2042: f64, t22417: f64, t22434: f64, t22439: f64, t22655: f64, t22657: f64, t22659: f64, t22661: f64, t22663: f64, t22666: f64) -> (f64, f64, f64, f64, f64) {
    let t22669 = 0.65061485296689145287e-1_f64 * t22668;
    let t22670 = t6742 * t6745;
    let t22671 = 0.13012297059337829057e0_f64 * t22670;
    let t22675 = 0.58482233974552040708e0_f64 * t601 * t580 * t22531 * t587;
    let t22676 = t2042 * t1963;
    let t22677 = 120.0_f64 * t22676;
    let t22678 = -t22655 - t22417 + t22434 - t22439 + t22657 - t22659 + t22661 - t22663 + t22666 + t22669 - t22671 - t22675 + t22677;
    (t22669, t22671, t22675, t22677, t22678)
}
