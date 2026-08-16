//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta829 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2687;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2688;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta829(t1062: f64, t19857: f64, t15745: f64, t4845: f64, t11859: f64, t11922: f64, t20074: f64, t15926: f64, t16035: f64, t11927: f64, t19830: f64, t16055: f64, t19738: f64, t16095: f64, t20100: f64, t43131: f64, t20069: f64, t4899: f64, t20065: f64, t4892: f64, t15688: f64, t16584: f64, t15731: f64, t4879: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t67269, t67301, t67327, t67329, t67353, t67355) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2687(t1062, t19857, t15745, t4845, t11859, t11922, t20074, t15926, t16035, t11927, t19830, t16055, t19738);
        let (t67358, t67426, t67435, t67458, t67473) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2688(t16095, t20100, t43131, t11922, t20069, t4899, t20065, t4892, t15688, t16584, t15731, t4879);
    (t67269, t67301, t67327, t67329, t67353, t67355, t67358, t67426, t67435, t67458, t67473)
}
