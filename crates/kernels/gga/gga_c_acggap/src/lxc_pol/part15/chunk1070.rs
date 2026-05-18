//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1070/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1070<F: Float>(t2131: F, t2147: F, t309: F, t9417: F, t463: F, t9431: F, t2132: F, t2138: F, t322: F, t9367: F, t8073: F, t8397: F) -> (F, F, F, F) {
    let t38153 = F::new(0.34694512752820797848e1) * t2131 * t2147 * t9417 * t309;
    let t38157 = F::new(0.34694512752820797848e1) * t2131 * t2147 * t9431 * t463;
    let t38165 = F::new(0.17347256376410398924e1) * t2138 * t2132 * t9367 * t322;
    let t38176 = F::new(0.34694512752820797848e1) * t8397 * t8073;
    (t38153, t38157, t38165, t38176)
}
