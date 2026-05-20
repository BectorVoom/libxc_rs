//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta829 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2687;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2688;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta829<F: Float>(t1062: F, t19857: F, t15745: F, t4845: F, t11859: F, t11922: F, t20074: F, t15926: F, t16035: F, t11927: F, t19830: F, t16055: F, t19738: F, t16095: F, t20100: F, t43131: F, t20069: F, t4899: F, t20065: F, t4892: F, t15688: F, t16584: F, t15731: F, t4879: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t67269, t67301, t67327, t67329, t67353, t67355) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2687::<F>(t1062, t19857, t15745, t4845, t11859, t11922, t20074, t15926, t16035, t11927, t19830, t16055, t19738);
        let (t67358, t67426, t67435, t67458, t67473) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2688::<F>(t16095, t20100, t43131, t11922, t20069, t4899, t20065, t4892, t15688, t16584, t15731, t4879);
    (t67269, t67301, t67327, t67329, t67353, t67355, t67358, t67426, t67435, t67458, t67473)
}
