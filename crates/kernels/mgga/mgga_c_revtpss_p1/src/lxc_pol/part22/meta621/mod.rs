//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta621 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2530;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2531;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta621<F: Float>(t1045: F, t19477: F, t373: F, t1042: F, t18909: F, t4919: F, t1011: F, t1041: F, t11732: F, t11737: F, t15656: F, t15732: F, t15736: F, t15744: F, t15750: F, t15754: F, t1665: F, t4854: F, t4858: F, t19456: F, t247: F, t3116: F, t3172: F, t6311: F, t3161: F, t1043: F, t6244: F, t3117: F, t1668: F, t4772: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t19799, t19800, t19809, t19813) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2530::<F>(t1045, t19477, t373, t1042, t18909, t4919, t1011, t1041, t11732, t11737, t15656, t15732, t15736, t15744, t15750, t15754, t1665, t4854, t4858);
        let (t19819, t19826, t19827, t19829, t19830, t19831, t19836) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2531::<F>(t19456, t247, t3116, t3172, t6311, t3161, t1043, t6244, t1045, t3117, t1668, t4772);
    (t19799, t19800, t19809, t19813, t19819, t19826, t19827, t19829, t19830, t19831, t19836)
}
