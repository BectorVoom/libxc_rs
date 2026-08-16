//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1111/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1111(t1125: f64, t17955: f64, t757: f64, t2096: f64, t7581: f64, t154: f64, t2739: f64, t276: f64, t5688: f64, t17938: f64, t18290: f64, t2019: f64, t2956: f64) -> (f64, f64, f64, f64, f64) {
    let t21933 = t757 * t17955 * t1125;
    let t21935 = t2096 * t7581;
    let t21950 = t276 * t154 * t5688 * t2739;
    let t21951 = t21950 / 144.0_f64;
    let t22007 = t17938 * t18290;
    let t22082 = t2019 * t2956;
    (t21933, t21935, t21951, t22007, t22082)
}
