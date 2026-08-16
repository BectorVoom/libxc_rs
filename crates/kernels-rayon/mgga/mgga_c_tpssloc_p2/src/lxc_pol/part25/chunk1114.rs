//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1114/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1114(t1992: f64, t40475: f64, t550: f64, t6976: f64, t22897: f64, t3792: f64, t81028: f64, t22899: f64, t6914: f64, t22715: f64, t6887: f64, t6970: f64) -> (f64, f64, f64, f64, f64) {
    let t81177 = t1992 * t6976 * t40475 * t550;
    let t81181 = t1992 * t22897 * t81028 * t3792;
    let t81184 = t6914 * t22899;
    let t81186 = t22715 * t6887;
    let t81187 = t81186 * t6970;
    (t81177, t81181, t81184, t81186, t81187)
}
