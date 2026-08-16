//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta634 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2088;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta634<F: Float>(t15749: F, t7117: F, t25490: F, t4845: F, t15666: F, t27479: F, t3215: F, t25577: F, t4817: F, t15711: F, t7132: F, t15655: F, t1972: F) -> (F, F, F, F, F, F, F) {
        let (t100329, t100332, t100334, t100336, t100342, t100343, t100345) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2088::<F>(t15749, t7117, t25490, t4845, t15666, t27479, t3215, t25577, t4817, t15711, t7132, t15655, t1972);
    (t100329, t100332, t100334, t100336, t100342, t100343, t100345)
}
