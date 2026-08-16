//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta893 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2849;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2850;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta893<F: Float>(t61093: F, t50903: F, t6002: F, t14613: F, t18539: F, t18544: F, t4311: F, t23214: F, t750: F, t49897: F, t14386: F, t5999: F, t61114: F, t18569: F, t22671: F, t706: F, t39483: F, t39520: F, t39528: F, t39531: F, t39534: F, t39537: F, t39540: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t76944, t76946, t76948, t76950, t76951, t76952, t76954) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2849::<F>(t61093, t50903, t6002, t14613, t18539, t18544, t4311, t23214, t750, t49897, t14386, t5999);
        let (t76955, t76957, t76960, t76961) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2850::<F>(t61114, t18569, t4311, t22671, t706, t750, t39483, t39520, t39528, t39531, t39534, t39537, t39540, t76944, t76946, t76948, t76950, t76951, t76952, t76954);
    (t76944, t76946, t76948, t76950, t76951, t76952, t76954, t76955, t76957, t76960, t76961)
}
