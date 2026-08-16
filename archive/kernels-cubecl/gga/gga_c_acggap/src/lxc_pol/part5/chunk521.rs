//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 521/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk521<F: Float>(t2722: F, t2823: F, t60: F, t40: F, t276: F, t901: F, t1268: F, t469: F, t883: F, t912: F, t2773: F, t2775: F, t690: F) -> (F, F, F, F, F, F, F, F) {
    let t2824 = t2722 + t2823;
    let t2825 = t60 * t2824;
    let t2826 = t40 * t2825;
    let t2827 = t901 * t276;
    let t2828 = t40 * t2827;
    let t2831 = t1268 * t469;
    let t2835 = t883 * t912;
    let t2838 = t2773 * t2775 * t690;
    (t2824, t2825, t2826, t2827, t2828, t2831, t2835, t2838)
}
