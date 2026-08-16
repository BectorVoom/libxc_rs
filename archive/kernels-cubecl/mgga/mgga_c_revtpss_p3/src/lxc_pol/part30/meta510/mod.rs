//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta510 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1891;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta510<F: Float>(t7760: F, t786: F, t789: F, t231: F, t7759: F, t836: F, t7076: F, t27198: F, t867: F, t7060: F, t7063: F, t14685: F, t1941: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t27202, t27203, t27207, t27212, t27213, t27214, t27216, t27217, t27221) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1891::<F>(t7760, t786, t789, t231, t7759, t836, t7076, t27198, t867, t7060, t7063, t14685, t1941);
    (t27202, t27203, t27207, t27212, t27213, t27214, t27216, t27217, t27221)
}
