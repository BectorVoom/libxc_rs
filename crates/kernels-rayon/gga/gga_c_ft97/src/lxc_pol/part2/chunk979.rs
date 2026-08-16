//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 979/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk979(t4129: f64, t824: f64, t2862: f64, t319: f64, t12001: f64, t4159: f64, t2842: f64, t668: f64, t2844: f64, t992: f64, t2881: f64, t4241: f64, t681: f64, t89: f64) -> (f64, f64, f64, f64) {
    let t15175 = t4129 * t824;
    let t15177 = t2862 * t319 * t15175;
    let t15180 = t12001 * t4159;
    let t15182 = t2842 * t668;
    let t15183 = t992 * t2844;
    let t15184 = t15182 * t15183;
    let t15185 = t2881 * t15184;
    let t15190 = 2.0_f64 / 9.0_f64 * t89 * t681 * t4241;
    (t15177, t15180, t15185, t15190)
}
