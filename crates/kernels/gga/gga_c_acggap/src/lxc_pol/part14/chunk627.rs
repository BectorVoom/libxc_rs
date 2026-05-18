//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 627/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk627<F: Float>(t2841: F, t4057: F, t4069: F, t5474: F, t87: F, t40: F, t2655: F, t2658: F, t2669: F, t2695: F, t2840: F, t4044: F, t4046: F, t4049: F, t4050: F, t4061: F, t4063: F, t5479: F) -> F {
    let t6005 = F::new(8.0) * t2841;
    let t6006 = F::new(16.0) * t4057;
    let t6007 = F::new(2.0) * t4069;
    let t6008 = t5474 * t87;
    let t6009 = t40 * t6008;
    let t6010 = t2655 - t2658 + t5479 + t2840 - t6005 + t4044 + t2669 + t2695 - t4046 - t4049 - t4050 - t6006 + t4061 - t4063 + t6007 + t6009;
    t6010
}
