//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta524 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1950;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1951;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta524<F: Float>(t33: F, t265: F, t502: F, t27754: F, t1469: F, t2003: F, t27821: F, t4186: F, t57: F, t606: F, t7215: F, t7877: F, t27762: F, t196: F, t197: F, t5528: F, dens_threshold: F, rho1: F, zeta_threshold: F, t2035: F, t7313: F, t7898: F, t1032: F, t1892: F, t1955: F) -> (F, F, F, F, F, F, F) {
        let (t27822, t27830, t27833) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1950::<F>(t33, t265, t502, t27754, t1469, t2003, t27821, t4186, t57, t606, t7215, t7877, t27762, t196, t197, t5528, dens_threshold, rho1, zeta_threshold);
        let (t27834, t27835, t27836, t27837) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1951::<F>(t2035, t27833, t7313, t7898, t1032, t1892, t1955);
    (t27822, t27830, t27833, t27834, t27835, t27836, t27837)
}
