//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta508 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1836;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1837;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1838;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta508<F: Float>(t1580: F, t7014: F, t689: F, t27279: F, t7058: F, t72: F, t7769: F, t686: F, t25375: F, t25387: F, t1559: F, t886: F, t25392: F, t1955: F, t7057: F, t14495: F, t1949: F, t2718: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t27334, t27335, t27338, t27340, t27341) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1836::<F>(t1580, t7014, t689, t27279, t7058, t72, t7769, t686);
        let (t27342, t27344, t27349, t27350, t27353) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1837::<F>(t25375, t27341, t25387, t1559, t886, t25392, t1955, t7057);
        let (t27354, t27357) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1838::<F>(t14495, t25392, t1949, t2718);
    (t27334, t27335, t27338, t27340, t27341, t27342, t27344, t27349, t27350, t27353, t27354, t27357)
}
