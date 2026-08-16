//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 740/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk740(t1939: f64, t247: f64, t1915: f64, t690: f64, t1954: f64, t709: f64, t2020: f64, t5712: f64, t5717: f64, t750: f64) -> (f64, f64, f64, f64, f64) {
    let t5873 = 1.0_f64 / t1939 / t247;
    let t5897 = t690 * t1915;
    let t5903 = t709 * t1954;
    let t5925 = t2020 * t5712;
    let t5931 = t5717 * t750;
    (t5873, t5897, t5903, t5925, t5931)
}
