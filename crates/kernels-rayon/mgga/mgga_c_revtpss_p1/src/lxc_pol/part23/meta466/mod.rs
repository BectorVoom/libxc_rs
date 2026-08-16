//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta466 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1908;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1909;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1910;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta466(t1042: f64, t19791: f64, t1025: f64, t1028: f64, t15618: f64, t15712: f64, t15724: f64, t19770: f64, t19773: f64, t19778: f64, t19782: f64, t19786: f64, t3091: f64, t3124: f64, t3127: f64, t3224: f64, t4788: f64, t6278: f64, t6302: f64, t1045: f64, t19477: f64, t373: f64, t18909: f64, t4919: f64, t1011: f64, t1041: f64, t11732: f64, t11737: f64, t15656: f64, t15732: f64, t15736: f64, t15744: f64, t15750: f64, t15754: f64, t1665: f64, t4854: f64, t4858: f64, t19456: f64, t247: f64, t3116: f64, t3172: f64, t6311: f64, t3161: f64, t1043: f64, t6244: f64, t3117: f64, t1668: f64, t4772: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t19792, t19797) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1908(t1042, t19791, t1025, t1028, t15618, t15712, t15724, t19770, t19773, t19778, t19782, t19786, t3091, t3124, t3127, t3224, t4788, t6278, t6302);
        let (t19799, t19800, t19813) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1909(t1045, t19477, t373, t1042, t18909, t4919, t1011, t1041, t11732, t11737, t15656, t15732, t15736, t15744, t15750, t15754, t1665, t4854, t4858);
        let (t19819, t19826, t19827, t19829, t19830, t19831, t19836) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1910(t19456, t247, t3116, t3172, t6311, t3161, t1043, t6244, t1045, t3117, t1668, t4772);
    (t19792, t19797, t19799, t19800, t19813, t19819, t19826, t19827, t19829, t19830, t19831, t19836)
}
