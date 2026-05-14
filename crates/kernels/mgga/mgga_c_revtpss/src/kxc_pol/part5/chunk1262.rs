//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1262/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1262<F: Float>(t1320: F, t6801: F, t189: F, t21931: F, t512: F, t6800: F, t749: F, t13611: F, t13621: F, t9398: F, t9406: F, t13630: F, t13633: F, t13615: F, t13620: F, t13623: F, t13634: F, t13635: F, t9394: F, t9415: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t22191 = t1320 * t6801;
    let t22192 = 4.0 * t22191;
    let t22193 = t21931 * t189;
    let t22194 = t512 * t22193;
    let t22195 = t6800 * t749;
    let t22196 = t512 * t22195;
    let t22197 = 0.11696447245269292414e1 * t13611;
    let t22198 = 16.0 * t13621;
    let t22199 = 8.0 * t9398;
    let t22200 = 8.0 * t9406;
    let t22201 = 0.23392894490538584828e1 * t13630;
    let t22202 = 2.0 * t13633;
    let t22203 = -t22192 + t22194 + t22196 - t22197 - t13615 + t9394 - t13620 - t22198 - t13623 - t22199 - t22200 + t22201 + t22202 + t13634 - t13635 - t9415;
    (t22192, t22194, t22196, t22197, t22198, t22199, t22200, t22201, t22202, t22203)
}
