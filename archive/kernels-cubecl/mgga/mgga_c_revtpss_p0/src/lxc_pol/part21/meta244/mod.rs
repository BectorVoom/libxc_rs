//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta244 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1423;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1424;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta244<F: Float>(t1317: F, t3855: F, t4029: F, t1333: F, t3863: F, t27: F, t583: F, t521: F, t19: F, t596: F, t182: F, t2490: F, t2495: F, t9368: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t9404, t9405, t9407, t9408, t9409, t9410, t9411, t9412, t9413, t9415, t9417) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1423::<F>(t1317, t3855, t4029, t1333, t3863, t27, t583, t521, t19, t596, t182, t2490);
        let t9419 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1424::<F>(t2495, t9368, t9417);
    (t9404, t9405, t9407, t9408, t9409, t9410, t9411, t9412, t9413, t9415, t9417, t9419)
}
