//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 1003/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk1003(t15541: f64, t2456: f64, t3787: f64, t7521: f64, t6851: f64, t871: f64, t2440: f64, t286: f64, t5: f64, t4: f64, t8139: f64, t8140: f64, t941: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15542 = t15541 * t2456;
    let t15548 = t3787 * t7521;
    let t15553 = t871 * t6851;
    let t15555 = t15541 * t2440;
    let t15608 = t5 * t286;
    let t15609 = t15608 * t4;
    let t15610 = t8139 * t15609;
    let t15615 = t941 * t8140;
    (t15542, t15548, t15553, t15555, t15609, t15610, t15615)
}
