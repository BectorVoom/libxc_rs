//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta518 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1919;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1920;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta518<F: Float>(t1444: F, t7920: F, t25924: F, t1398: F, t543: F, t7910: F, t7301: F, t1882: F, t7274: F, t2022: F, t5658: F, t26054: F, t5722: F, t1883: F, t25931: F, t1955: F, t7283: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t27840, t27841, t27845, t27846, t27852, t27853, t27857, t27858, t27861) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1919::<F>(t1444, t7920, t25924, t1398, t543, t7910, t7301, t1882, t7274, t2022, t5658, t26054, t5722);
        let (t27864, t27865, t27868) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1920::<F>(t1444, t1883, t25931, t1955, t7283);
    (t27840, t27841, t27845, t27846, t27852, t27853, t27857, t27858, t27861, t27864, t27865, t27868)
}
