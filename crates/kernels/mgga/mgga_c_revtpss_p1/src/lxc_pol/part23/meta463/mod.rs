//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta463 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1902;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1903;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1904;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta463<F: Float>(t19450: F, t4900: F, t3117: F, t11661: F, t19501: F, t3092: F, t1047: F, t1063: F, t12013: F, t16067: F, t16089: F, t19688: F, t19693: F, t19697: F, t19702: F, t19707: F, t19718: F, t3127: F, t4803: F, t4808: F, t4834: F, t4892: F, t4899: F, t6308: F, t15957: F, t6266: F, t16509: F, t4891: F, t16584: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t19721, t19722, t19725, t19726, t19729) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1902::<F>(t19450, t4900, t3117, t11661, t19501, t3092, t1047, t1063, t12013, t16067, t16089, t19688, t19693, t19697, t19702, t19707, t19718, t3127, t4803, t4808, t4834, t4892, t4899, t6308);
        let (t19730, t19731, t19738) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1903::<F>(t15957, t6266, t3092, t16509, t4891);
        let t19741 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1904::<F>(t16584, t4891);
    (t19721, t19722, t19725, t19726, t19729, t19730, t19731, t19738, t19741)
}
