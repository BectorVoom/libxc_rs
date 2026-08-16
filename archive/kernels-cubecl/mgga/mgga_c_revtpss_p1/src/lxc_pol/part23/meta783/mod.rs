//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta783 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2592;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta783<F: Float>(t1774: F, t487: F, t45928: F, t45934: F, t45938: F, t45945: F, t45949: F, t2246: F, t4171: F, t10308: F, t1466: F, t10355: F, t44: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t60037, t60214, t60215, t60216, t60217, t60218, t60221, t60224, t60308) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2592::<F>(t1774, t487, t45928, t45934, t45938, t45945, t45949, t2246, t4171, t10308, t1466, t10355, t44);
    (t60037, t60214, t60215, t60216, t60217, t60218, t60221, t60224, t60308)
}
