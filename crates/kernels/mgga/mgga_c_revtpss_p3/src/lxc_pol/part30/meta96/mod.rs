//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta96 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk613;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk614;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk615;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk616;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta96<F: Float>(t33: F, t2002: F, t2159: F, t57: F, t2132: F, dens_threshold: F, rho1: F, zeta_threshold: F, t2010: F, t2127: F, t118: F, t1939: F, t2036: F, t508: F, t569: F, t3: F, param_d: F, t2044: F, t573: F, t10: F, t17: F, t576: F, t580: F, t15: F, t22: F, t11: F, t14: F, t584: F, t588: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t2163 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk613::<F>(t33, t2002, t2159, t57, t2132, dens_threshold, rho1, zeta_threshold);
        let t2165 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk614::<F>(t2010, t2127);
        let (t2167, t2168, t2170) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk615::<F>(t118, t1939, t2036, t2127, t2163, t2165, t508, t569, t3, param_d);
        let (t2172, t2219, t2221, t2223, t2224, t2226, t2228) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk616::<F>(t2044, t2170, t573, t10, t17, t576, t580, t15, t22, t11, t14, t584, t588);
    (t2163, t2165, t2167, t2168, t2170, t2172, t2219, t2221, t2223, t2224, t2226, t2228)
}
