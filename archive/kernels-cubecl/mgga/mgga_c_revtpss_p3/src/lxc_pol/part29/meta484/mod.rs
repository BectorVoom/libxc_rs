//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta484 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1767;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta484<F: Float>(t27940: F, t5677: F, t26028: F, t5697: F, t5701: F, t5706: F, t5614: F, t7271: F, t5661: F, t7264: F, t25997: F, t5665: F) -> (F, F, F, F, F, F, F) {
        let (t27941, t27943, t27945, t27947, t27949, t27951, t27953) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1767::<F>(t27940, t5677, t26028, t5697, t5701, t5706, t5614, t7271, t5661, t7264, t25997, t5665);
    (t27941, t27943, t27945, t27947, t27949, t27951, t27953)
}
