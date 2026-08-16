//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta377 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1414;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta377<F: Float>(t1065: F, t1668: F, t372: F, t4823: F, t1087: F, t11773: F, t1062: F, t4857: F, t11986: F, t1592: F, t247: F, t1063: F) -> (F, F, F, F, F, F) {
        let (t15691, t15696, t15700, t15707, t15711, t15712) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1414::<F>(t1065, t1668, t372, t4823, t1087, t11773, t1062, t4857, t11986, t1592, t247, t1063);
    (t15691, t15696, t15700, t15707, t15711, t15712)
}
