//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta469 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1917;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1918;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta469(t19691: f64, t4801: f64, t1042: f64, t140: f64, t6284: f64, t1011: f64, t6288: f64, t6292: f64, t1015: f64, t18281: f64, t1012: f64, t3172: f64, t6262: f64, t3127: f64, t11881: f64, t15986: f64, t15990: f64, t15996: f64, t16037: f64, t3241: f64, t6289: f64, t6293: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t19894, t19895, t19900, t19901, t19907, t19908, t19912, t19913, t19916, t19917, t19920) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1917(t19691, t4801, t1042, t140, t6284, t1011, t6288, t6292, t1015, t18281, t1012, t3172, t6262);
        let (t19921, t19923) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1918(t19920, t3127, t1011, t11881, t15986, t15990, t15996, t16037, t19908, t19913, t19917, t3241, t6289, t6293);
    (t19894, t19895, t19900, t19901, t19907, t19908, t19912, t19913, t19916, t19920, t19921, t19923)
}
