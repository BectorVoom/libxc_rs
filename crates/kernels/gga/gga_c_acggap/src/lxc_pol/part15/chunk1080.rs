//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1080/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1080<F: Float>(t8100: F, t8397: F, t8061: F, t8998: F, t2138: F, t2147: F, t322: F, t9417: F, t1717: F, t467: F, t301: F, t1662: F, t560: F) -> (F, F, F, F, F, F) {
    let t38487 = F::new(0.17347256376410398924e1) * t8397 * t8100;
    let t38489 = F::new(0.17347256376410398924e1) * t8998 * t8061;
    let t38493 = F::new(0.34694512752820797848e1) * t2138 * t2147 * t9417 * t322;
    let t38519 = t1717 * t467;
    let t38534 = t1717 * t301;
    let t38540 = t560 * t1662;
    (t38487, t38489, t38493, t38519, t38534, t38540)
}
