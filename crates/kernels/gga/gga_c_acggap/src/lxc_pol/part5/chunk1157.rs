//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1157/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1157<F: Float>(t1165: F, t3194: F, t5284: F, t5862: F, t5961: F, t997: F, t1036: F, t1426: F, t1713: F, t175: F, t864: F, t1032: F, t5826: F) -> (F, F, F, F) {
    let t20875 = t3194 * t1165 * t5862 * t5284;
    let t20882 = t997 * t5961;
    let t20888 = t1036 * t1426 * t175 * t1713 * t864;
    let t20890 = t1032 * t5826;
    (t20875, t20882, t20888, t20890)
}
