//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1080/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1080(t15541: f64, t2440: f64, t286: f64, t5: f64, t4: f64, t8139: f64, t8140: f64, t941: f64, t186: f64, t2786: f64, t2579: f64, t923: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15555 = t15541 * t2440;
    let t15608 = t5 * t286;
    let t15609 = t15608 * t4;
    let t15610 = t8139 * t15609;
    let t15615 = t941 * t8140;
    let t15644 = t2786 * t186;
    let t15650 = t2579 * t923;
    (t15555, t15609, t15610, t15615, t15644, t15650)
}
