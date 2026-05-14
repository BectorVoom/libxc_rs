//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 948/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk948<F: Float>(t2132: F, t2385: F, t7885: F, t864: F, t8104: F, t8397: F, t2138: F, t2147: F, t463: F, t9435: F, t7987: F, t9432: F, t2131: F, t309: F, t9417: F, t9431: F) -> (F, F, F, F, F, F) {
    let t38104 = t7885 * t2132 * t2385 * t864;
    let t38111 = t8397 * t8104;
    let t38138 = 0.34694512752820797848e1 * t2138 * t2147 * t9435 * t463;
    let t38140 = 0.17347256376410398924e1 * t7987 * t9432;
    let t38153 = 0.34694512752820797848e1 * t2131 * t2147 * t9417 * t309;
    let t38157 = 0.34694512752820797848e1 * t2131 * t2147 * t9431 * t463;
    (t38104, t38111, t38138, t38140, t38153, t38157)
}
