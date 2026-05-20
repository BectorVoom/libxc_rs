//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta328 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1342;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta328<F: Float>(t10981: F, t10982: F, t2455: F, t9285: F, t2454: F, t2829: F, t779: F, t689: F, t2444: F, t887: F, t252: F, t2769: F) -> (F, F, F, F, F, F) {
        let (t10984, t10985, t10987, t10989, t10992, t10994) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1342::<F>(t10981, t10982, t2455, t9285, t2454, t2829, t779, t689, t2444, t887, t252, t2769);
    (t10984, t10985, t10987, t10989, t10992, t10994)
}
