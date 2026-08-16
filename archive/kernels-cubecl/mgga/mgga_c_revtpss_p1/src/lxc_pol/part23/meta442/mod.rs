//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta442 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1859;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta442<F: Float>(t19127: F, t935: F, t915: F, t11294: F, t6145: F, t11465: F, t6189: F, t4733: F, t981: F, t11108: F, t6400: F, t1100: F, t18902: F, t19025: F, t19027: F, t19029: F, t19031: F, t19048: F, t19051: F, t19053: F, t19055: F, t19058: F, t19060: F, t19062: F, t19079: F, t19081: F, t19084: F, t5023: F) -> (F, F, F, F, F, F, F, F) {
        let (t19128, t19130, t19132, t19133, t19134, t19136, t19137, t19141) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1859::<F>(t19127, t935, t915, t11294, t6145, t11465, t6189, t4733, t981, t11108, t6400, t1100, t18902, t19025, t19027, t19029, t19031, t19048, t19051, t19053, t19055, t19058, t19060, t19062, t19079, t19081, t19084, t5023);
    (t19128, t19130, t19132, t19133, t19134, t19136, t19137, t19141)
}
