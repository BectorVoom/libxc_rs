//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 981/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk981(t2682: f64, t992: f64, t2875: f64, t10492: f64, t1882: f64, t4305: f64, t1255: f64, t2862: f64, t2739: f64, t840: f64, t4129: f64, t882: f64) -> (f64, f64, f64, f64, f64) {
    let t15200 = t992 * t2682;
    let t15201 = t2875 * t15200;
    let t15202 = t10492 * t15201;
    let t15206 = 2.0_f64 / 9.0_f64 * t1882 * t4305;
    let t15208 = t2862 * t1255 * t2682;
    let t15212 = t840 * t1255 * t2739;
    let t15218 = t840 * t882 * t4129;
    (t15202, t15206, t15208, t15212, t15218)
}
