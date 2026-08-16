//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta519 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2155;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta519<F: Float>(t15717: F, t996: F, t1678: F, t3057: F, t15648: F, t16152: F, t15837: F, t4930: F, t994: F, t3046: F, t1000: F, t11187: F, t11201: F, t11220: F, t1680: F, t1696: F, t3043: F, t3047: F, t3058: F, t3060: F, t3063: F, t3264: F, t3271: F, t4752: F, t4758: F, t4764: F, t4773: F, t4941: F, t4947: F, t995: F) -> (F, F, F, F, F, F, F, F) {
        let (t16275, t16284, t16287, t16292, t16295, t16302, t16305, t16310) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2155::<F>(t15717, t996, t1678, t3057, t15648, t16152, t15837, t4930, t994, t3046, t1000, t11187, t11201, t11220, t1680, t1696, t3043, t3047, t3058, t3060, t3063, t3264, t3271, t4752, t4758, t4764, t4773, t4941, t4947, t995);
    (t16275, t16284, t16287, t16292, t16295, t16302, t16305, t16310)
}
