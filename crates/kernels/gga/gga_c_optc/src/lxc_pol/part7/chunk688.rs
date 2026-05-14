//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 688/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk688<F: Float>(t2067: F, t645: F, t127: F, t162: F, t1948: F, t2035: F, t2034: F, t2022: F, t2024: F, t616: F, t2030: F, t2037: F, t2074: F, t2020: F, t2029: F, t2026: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t6776 = t2067 * t645;
    let t6777 = t6776 * t127;
    let t6778 = t162 * t6777;
    let t6781 = t2035 * t1948;
    let t6782 = t2034 * t6781;
    let t6785 = t2022 * t2024;
    let t6786 = t6785 * t616;
    let t6787 = t2034 * t6786;
    let t6790 = t645 * t2024;
    let t6791 = t6790 * t2067;
    let t6792 = t162 * t6791;
    let t6795 = t2030 * t2037;
    let t6797 = t2030 * t2074;
    let t6799 = t2020 * t2029;
    let t6800 = t6799 * t2026;
    (t6777, t6778, t6781, t6782, t6785, t6786, t6787, t6791, t6792, t6795, t6797, t6799, t6800)
}
