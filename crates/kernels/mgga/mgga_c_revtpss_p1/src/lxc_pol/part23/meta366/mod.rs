//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta366 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1684;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta366<F: Float>(t15125: F, t15168: F, t15191: F, t15197: F, t15127: F, t2986: F, t4707: F, t300: F, t4682: F, t3215: F, t4858: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t15435, t15447, t15457, t15459, t15483, t15484, t15485, t15503, t15504, t15537, t15547, t15583) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1684::<F>(t15125, t15168, t15191, t15197, t15127, t2986, t4707, t300, t4682, t3215, t4858);
    (t15435, t15447, t15457, t15459, t15483, t15484, t15485, t15503, t15504, t15537, t15547, t15583)
}
