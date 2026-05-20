//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta358 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1672;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta358<F: Float>(t4469: F, t822: F, t4533: F, t72: F, t686: F, t2465: F, t1569: F, t867: F, t786: F) -> (F, F, F, F, F, F) {
        let (t14972, t14982, t14983, t14985, t14986, t14987) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1672::<F>(t4469, t822, t4533, t72, t686, t2465, t1569, t867, t786);
    (t14972, t14982, t14983, t14985, t14986, t14987)
}
