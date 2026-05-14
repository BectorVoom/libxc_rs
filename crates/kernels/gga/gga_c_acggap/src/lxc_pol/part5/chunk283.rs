//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 283/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk283<F: Float>(t147: F, t987: F, t165: F, t335: F, t397: F, t932: F, t936: F, t942: F, t947: F, t953: F, t957: F, t962: F, t968: F, t976: F, t979: F, t983: F) -> (F, F) {
    let t989 = 35.0 / 432.0 * t987 * t147;
    let t990 = -0.21437009059034868486e-3 * t397 * t932 - 0.42874018118069736972e-3 * t936 + 0.42874018118069736972e-3 * t942 * t947 + 0.20007875121765877254e-2 * t953 - 0.21437009059034868486e-3 * t397 * t957 + t335 * t962 / 24.0 + 0.42874018118069736972e-3 * t165 * t968 + t976 - t979 + t983 + t989;
    (t989, t990)
}
