//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta579 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1906;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1907;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta579<F: Float>(t1445: F, t28824: F, t689: F, t102274: F, t25878: F, t102100: F, t26069: F, t26231: F, t98380: F, t13730: F, t2098: F, t2782: F, t102315: F, t25899: F, t2439: F, t8099: F, t94391: F, t102234: F, t3916: F, t25895: F, t2097: F, t9990: F, t102115: F, t7289: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t102361, t102363, t102364, t102367, t102372) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1906::<F>(t1445, t28824, t689, t102274, t25878, t102100, t26069, t26231, t98380, t13730, t2098, t2782);
        let (t102378, t102385, t102386, t102394, t102396, t102397, t102404) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1907::<F>(t102315, t25899, t2439, t8099, t94391, t102234, t3916, t25895, t2097, t9990, t102115, t7289);
    (t102361, t102363, t102364, t102367, t102372, t102378, t102385, t102386, t102394, t102396, t102397, t102404)
}
