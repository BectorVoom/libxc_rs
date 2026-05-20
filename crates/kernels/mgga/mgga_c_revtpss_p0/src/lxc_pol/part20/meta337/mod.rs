//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta337 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1261;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta337<F: Float>(t225: F, t9990: F, t213: F, t10605: F, t162: F, t10439: F, t2394: F, t262: F, t10867: F, t10871: F, t2722: F, t73: F, t830: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t14192, t14193, t14325, t14330, t14375, t14545, t14546, t14547, t14643) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1261::<F>(t225, t9990, t213, t10605, t162, t10439, t2394, t262, t10867, t10871, t2722, t73, t830);
    (t14192, t14193, t14325, t14330, t14375, t14545, t14546, t14547, t14643)
}
