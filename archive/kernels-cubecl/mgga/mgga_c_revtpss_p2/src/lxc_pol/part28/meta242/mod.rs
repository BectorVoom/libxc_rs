//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta242 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1093;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta242<F: Float>(t460: F, t5462: F, t3302: F, t3603: F, t1248: F, t5332: F, t1269: F, t1287: F, t1794: F, t487: F, t5284: F, t3781: F) -> (F, F, F, F, F, F, F) {
        let (t5463, t5464, t5465, t5466, t5470, t5474, t5477) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1093::<F>(t460, t5462, t3302, t3603, t1248, t5332, t1269, t1287, t1794, t487, t5284, t3781);
    (t5463, t5464, t5465, t5466, t5470, t5474, t5477)
}
