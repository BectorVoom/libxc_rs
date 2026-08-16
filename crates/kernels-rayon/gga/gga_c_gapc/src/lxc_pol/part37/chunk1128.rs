//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1128/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1128(t7556: f64, t966: f64, t9864: f64, t314: f64, t6: f64, t959: f64, t1038: f64, t19159: f64, t3787: f64, t2546: f64, t286: f64, t2553: f64, t3074: f64, t4: f64, t8133: f64) -> (f64, f64, f64, f64, f64) {
    let t29228 = t7556 * t966 * t9864;
    let t29314 = t6 * t959 * t314;
    let t29350 = t3787 * t1038 * t19159;
    let t29435 = t2546 * t286;
    let t29473 = t2553 * t3074 * t8133 * t4;
    (t29228, t29314, t29350, t29435, t29473)
}
