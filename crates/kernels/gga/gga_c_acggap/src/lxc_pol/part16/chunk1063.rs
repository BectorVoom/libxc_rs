//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1063/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1063<F: Float>(t1815: F, t322: F, t1181: F, t31562: F, t599: F, t157: F, t1838: F, t406: F, t1165: F, t2068: F, t604: F, t301: F) -> (F, F, F, F, F) {
    let t38778 = t1815 * t322;
    let t38781 = t31562 * t1181 * t599 * t38778;
    let t38784 = t1838 * t406 * t157;
    let t38787 = t2068 * t1165 * t604 * t38784;
    let t38789 = t1815 * t301;
    (t38778, t38781, t38784, t38787, t38789)
}
