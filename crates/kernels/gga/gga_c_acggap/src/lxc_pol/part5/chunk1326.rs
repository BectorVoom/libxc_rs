//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1326/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1326<F: Float>(t19395: F, t19401: F, t19424: F, t19438: F, t19514: F, t24559: F, t24564: F, t24571: F, t24578: F, t24587: F, t24601: F, t24617: F, t24633: F, t24643: F, t24654: F, t24655: F, t4055: F, t4057: F, t4060: F, t4062: F, t4065: F, t4069: F, t4101: F, t5395: F, t6009: F, t6013: F, t6581: F, t6585: F, t7: F) -> F {
    let t24662 = F::new(12.0) * t6581 - F::new(6.0) * t4055 - F::new(48.0) * t4057 + F::new(8.0) * t4060 - F::new(32.0) * t4062 + F::new(2.0) * t4065 + F::new(6.0) * t4069 + F::new(2.0) * t6009 + F::new(2.0) * t6013 + F::new(6.0) * t4101 + F::new(2.0) * t5395 + t7 * (t19395 + t19401 + t19424 + t19438 + t19514 + t24559 + t24564 + t24571 + t24578 + t24587 + t24601 + t24617 + t24633 + t24643 + t24654 + t24655) - F::new(6.0) * t6585;
    t24662
}
