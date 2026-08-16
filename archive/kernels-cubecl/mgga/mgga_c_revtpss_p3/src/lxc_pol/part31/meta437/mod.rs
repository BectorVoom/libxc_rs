//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta437 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1561;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1562;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1563;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1564;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta437<F: Float>(t1045: F, t19477: F, t373: F, t1042: F, t18909: F, t4919: F, t1011: F, t1041: F, t11732: F, t11737: F, t15656: F, t15732: F, t15736: F, t15744: F, t15750: F, t15754: F, t1665: F, t4854: F, t4858: F, t19456: F, t247: F, t3116: F, t3172: F, t6311: F, t3161: F, t1043: F, t6244: F, t3117: F, t1668: F, t4772: F, t11866: F, t11927: F, t15716: F, t15771: F, t15774: F, t15776: F, t15817: F, t1671: F, t3115: F, t4831: F, t4834: F, t4869: F, t4879: F, t6273: F, t11134: F, t11890: F, t15189: F, t15874: F, t15875: F, t15876: F, t18906: F, t18911: F, t18915: F, t18919: F, t18924: F, t18928: F, t18932: F, t18934: F, t18939: F, t18944: F, t18948: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t19800, t19813) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1561::<F>(t1045, t19477, t373, t1042, t18909, t4919, t1011, t1041, t11732, t11737, t15656, t15732, t15736, t15744, t15750, t15754, t1665, t4854, t4858);
        let (t19819, t19826, t19827, t19829, t19831, t19836) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1562::<F>(t19456, t247, t3116, t3172, t6311, t3161, t1043, t6244, t1045, t3117, t1668, t4772);
        let (t19838, t19841) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1563::<F>(t1045, t19836, t3117, t11866, t11927, t15716, t15771, t15774, t15776, t15817, t1671, t19819, t19827, t19831, t3115, t4831, t4834, t4869, t4879, t6273);
        let t19855 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1564::<F>(t11134, t11890, t15189, t15874, t15875, t15876, t18906, t18911, t18915, t18919, t18924, t18928, t18932, t18934, t18939, t18944, t18948);
    (t19800, t19813, t19819, t19826, t19829, t19831, t19836, t19838, t19841, t19855)
}
