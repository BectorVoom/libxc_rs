//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta264 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1111;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta264<F: Float>(t1045: F, t11620: F, t373: F, t1042: F, t1034: F, t360: F, t11244: F, t11240: F, t3154: F, t357: F) -> (F, F, F, F, F, F, F, F) {
        let (t11622, t11623, t11626, t11627, t11628, t11629, t11630, t11631) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1111::<F>(t1045, t11620, t373, t1042, t1034, t360, t11244, t11240, t3154, t357);
    (t11622, t11623, t11626, t11627, t11628, t11629, t11630, t11631)
}
