//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta279 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1231;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta279<F: Float>(t7301: F, t7925: F, t545: F, t7910: F, t2028: F, t1904: F, t2027: F, t2030: F, t213: F, t561: F, t7245: F, t7248: F, t7279: F, t7288: F, t7291: F, t7295: F, t7911: F, t7917: F, t7921: F) -> (F, F, F, F) {
        let (t7926, t7929, t7930, t7933) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1231::<F>(t7301, t7925, t545, t7910, t2028, t1904, t2027, t2030, t213, t561, t7245, t7248, t7279, t7288, t7291, t7295, t7911, t7917, t7921);
    (t7926, t7929, t7930, t7933)
}
