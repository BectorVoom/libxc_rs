//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2879/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2879<F: Float>(t1583: F, t1940: F, t39783: F, t39786: F, t39791: F, t39795: F, t39799: F, t49958: F, t49964: F, t49982: F, t63160: F, t76974: F, t76976: F) -> F {
    let t77386 = -F::new(3.0) * t1583 * t1940 * t63160 - t39783 - t39786 - t39791 - t39795 + t39799 - t49958 - t49964 + t49982 + t76974 + t76976;
    t77386
}
