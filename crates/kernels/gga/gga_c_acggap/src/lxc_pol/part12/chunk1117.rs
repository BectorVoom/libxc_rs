//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1117/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1117<F: Float>(t2030: F, t507: F, t7807: F, t1488: F, t2060: F, t2061: F, t1165: F, t20817: F, t604: F, t7337: F, t4975: F, t7561: F) -> (F, F, F, F) {
    let t35856 = t2030 * t507 * t7807;
    let t35860 = t2060 * t1488 * t2061;
    let t35864 = t7337 * t1165 * t604 * t20817;
    let t35866 = t7561 * t4975;
    (t35856, t35860, t35864, t35866)
}
