//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta469 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1917;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1918;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta469<F: Float>(t19691: F, t4801: F, t1042: F, t140: F, t6284: F, t1011: F, t6288: F, t6292: F, t1015: F, t18281: F, t1012: F, t3172: F, t6262: F, t3127: F, t11881: F, t15986: F, t15990: F, t15996: F, t16037: F, t3241: F, t6289: F, t6293: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t19894, t19895, t19900, t19901, t19907, t19908, t19912, t19913, t19916, t19917, t19920) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1917::<F>(t19691, t4801, t1042, t140, t6284, t1011, t6288, t6292, t1015, t18281, t1012, t3172, t6262);
        let (t19921, t19923) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1918::<F>(t19920, t3127, t1011, t11881, t15986, t15990, t15996, t16037, t19908, t19913, t19917, t3241, t6289, t6293);
    (t19894, t19895, t19900, t19901, t19907, t19908, t19912, t19913, t19916, t19920, t19921, t19923)
}
