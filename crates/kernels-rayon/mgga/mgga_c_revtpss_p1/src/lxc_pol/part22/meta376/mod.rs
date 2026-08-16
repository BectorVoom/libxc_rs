//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta376 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1927;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1928;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta376(t13312: f64, t48: f64, t10368: f64, t1469: f64, t2251: f64, t2282: f64, t4186: f64, t606: f64, t2258: f64, t4210: f64, t60: f64, t10379: f64, t13299: f64, t13303: f64, t13306: f64, t1474: f64, t1480: f64, t2270: f64, t2283: f64, t2286: f64, t4202: f64, t4205: f64, t44: f64, t56: f64, t614: f64, t38: f64, t1486: f64, t2259: f64, t4217: f64, t607: f64, t1471: f64, t1487: f64, t1494: f64, t2252: f64, t2260: f64, t2263: f64, t2312: f64, t4196: f64, t4218: f64, t4238: f64, t608: f64, t641: f64, t85: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13321, t13325, t13328, t13331, t13334) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1927(t13312, t48, t10368, t1469, t2251, t2282, t4186, t606, t2258, t4210, t60, t10379, t13299, t13303, t13306, t1474, t1480, t2270, t2283, t2286, t4202, t4205, t44, t56, t614);
        let (t13335, t13340, t13343, t13346, t13363) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1928(t13334, t38, t1486, t2251, t2259, t4217, t607, t1471, t1487, t1494, t2252, t2260, t2263, t2312, t4196, t4218, t4238, t608, t641, t85);
    (t13321, t13325, t13328, t13331, t13334, t13335, t13340, t13343, t13346, t13363)
}
