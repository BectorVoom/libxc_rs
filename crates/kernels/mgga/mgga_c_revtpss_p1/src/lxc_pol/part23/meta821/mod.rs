//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta821 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2671;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2672;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta821<F: Float>(t11999: F, t19826: F, t11262: F, t3150: F, t6307: F, t11710: F, t19725: F, t4892: F, t15669: F, t16088: F, t380: F, t1045: F, t4186: F, t1058: F, t19858: F, t15688: F, t16509: F, t19869: F, t3201: F, t6318: F, t1011: F, t15987: F, t18926: F, t18930: F, t15689: F, t19985: F, t53405: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t66024, t66029, t66043, t66047, t66066) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2671::<F>(t11999, t19826, t11262, t3150, t6307, t11710, t19725, t4892, t15669, t16088, t380, t1045, t4186);
        let (t66093, t66114, t66139, t66141, t66155, t66158, t66176) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2672::<F>(t1058, t19858, t15688, t16509, t19869, t3201, t6318, t1011, t15987, t18926, t18930, t15689, t19985, t53405);
    (t66024, t66029, t66043, t66047, t66066, t66093, t66114, t66139, t66141, t66155, t66158, t66176)
}
