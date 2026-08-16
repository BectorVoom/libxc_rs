//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1909/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1909(t1045: f64, t19477: f64, t373: f64, t1042: f64, t18909: f64, t4919: f64, t1011: f64, t1041: f64, t11732: f64, t11737: f64, t15656: f64, t15732: f64, t15736: f64, t15744: f64, t15750: f64, t15754: f64, t1665: f64, t4854: f64, t4858: f64) -> (f64, f64, f64) {
    let t19799 = t373 * t19477 * t1045;
    let t19800 = t1042 * t19799;
    let t19809 = t4919 * t18909;
    let t19813 = 0.21437009059034868486e-3_f64 * t1041 * t19800 - 0.95275595817932748827e-4_f64 * t15732 - t15736 - 0.42874018118069736972e-3_f64 * t15656 * t1665 - 0.42874018118069736972e-3_f64 * t4858 * t4854 + t15744 + 0.95275595817932748827e-4_f64 * t15750 - t1011 * t19809 / 36.0_f64 - t15754 + t11732 / 162.0_f64 + t11737;
    (t19799, t19800, t19813)
}
