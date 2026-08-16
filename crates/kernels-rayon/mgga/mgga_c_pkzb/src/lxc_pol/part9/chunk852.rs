//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 852/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk852(t6158: f64, t834: f64, t6087: f64, t336: f64, t6150: f64, t2215: f64, t836: f64, t2209: f64, t841: f64, t218: f64, t344: f64, t5555: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6159 = t834 * t6158;
    let t6161 = 0.93011851851851851854e0_f64 * t6087;
    let t6165 = 1.0_f64/pow_3_2(t336);
    let t6166 = t6165 * t6150;
    let t6168 = t2215 * t836;
    let t6169 = t6168 * t2209;
    let t6171 = t841 * t6158;
    let t6174 = t218 * t5555 * t344;
    (t6159, t6161, t6165, t6166, t6169, t6171, t6174)
}
