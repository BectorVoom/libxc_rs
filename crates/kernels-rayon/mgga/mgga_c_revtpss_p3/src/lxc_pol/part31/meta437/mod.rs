//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta437 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1561;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1562;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1563;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1564;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta437(t1045: f64, t19477: f64, t373: f64, t1042: f64, t18909: f64, t4919: f64, t1011: f64, t1041: f64, t11732: f64, t11737: f64, t15656: f64, t15732: f64, t15736: f64, t15744: f64, t15750: f64, t15754: f64, t1665: f64, t4854: f64, t4858: f64, t19456: f64, t247: f64, t3116: f64, t3172: f64, t6311: f64, t3161: f64, t1043: f64, t6244: f64, t3117: f64, t1668: f64, t4772: f64, t11866: f64, t11927: f64, t15716: f64, t15771: f64, t15774: f64, t15776: f64, t15817: f64, t1671: f64, t3115: f64, t4831: f64, t4834: f64, t4869: f64, t4879: f64, t6273: f64, t11134: f64, t11890: f64, t15189: f64, t15874: f64, t15875: f64, t15876: f64, t18906: f64, t18911: f64, t18915: f64, t18919: f64, t18924: f64, t18928: f64, t18932: f64, t18934: f64, t18939: f64, t18944: f64, t18948: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t19800, t19813) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1561(t1045, t19477, t373, t1042, t18909, t4919, t1011, t1041, t11732, t11737, t15656, t15732, t15736, t15744, t15750, t15754, t1665, t4854, t4858);
        let (t19819, t19826, t19827, t19829, t19831, t19836) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1562(t19456, t247, t3116, t3172, t6311, t3161, t1043, t6244, t1045, t3117, t1668, t4772);
        let (t19838, t19841) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1563(t1045, t19836, t3117, t11866, t11927, t15716, t15771, t15774, t15776, t15817, t1671, t19819, t19827, t19831, t3115, t4831, t4834, t4869, t4879, t6273);
        let t19855 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1564(t11134, t11890, t15189, t15874, t15875, t15876, t18906, t18911, t18915, t18919, t18924, t18928, t18932, t18934, t18939, t18944, t18948);
    (t19800, t19813, t19819, t19826, t19829, t19831, t19836, t19838, t19841, t19855)
}
