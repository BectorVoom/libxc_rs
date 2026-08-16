//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 824/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk824(t197: f64, t5931: f64, t5724: f64, t287: f64, t5728: f64, t5727: f64, t758: f64, t465: f64, t616: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5932 = t5931 * t197;
    let t5933 = t5932 * t5724;
    let t5934 = t5728 * t287;
    let t5935 = t5727 * t5934;
    let t5936 = t758 * t5935;
    let t5939 = t616 * t465;
    (t5932, t5933, t5934, t5935, t5936, t5939)
}
