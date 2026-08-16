//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta429 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1825;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta429<F: Float>(t6071: F, t72: F, t686: F, t2465: F, t213: F, t6041: F, t6048: F, t10995: F, t10987: F, t11000: F, t11004: F, t11013: F, t11017: F, t11019: F, t11030: F, t15018: F, t15047: F, t15050: F, t887: F) -> (F, F, F, F, F, F, F, F) {
        let (t18796, t18797, t18798, t18800, t18804, t18805, t18806, t18810) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1825::<F>(t6071, t72, t686, t2465, t213, t6041, t6048, t10995, t10987, t11000, t11004, t11013, t11017, t11019, t11030, t15018, t15047, t15050, t887);
    (t18796, t18797, t18798, t18800, t18804, t18805, t18806, t18810)
}
