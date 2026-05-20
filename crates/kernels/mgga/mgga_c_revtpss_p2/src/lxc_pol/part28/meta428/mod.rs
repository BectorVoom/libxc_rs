//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta428 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1611;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1612;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1613;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1614;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta428<F: Float>(t1651: F, t3133: F, t1045: F, t3117: F, t12167: F, t15905: F, t11631: F, t3151: F, t15907: F, t3057: F, t380: F, t3088: F, t370: F, t4757: F, t906: F, t3092: F, t994: F, t606: F, t999: F, t4578: F, t905: F, t15691: F, t11774: F, t11917: F, t11924: F, t11938: F, t11952: F, t11954: F, t11956: F, t11965: F, t3115: F, t3169: F, t4820: F, t1015: F, t13312: F, t1012: F, t4573: F, t11703: F, t3188: F, t4817: F, t1011: F, t11268: F, t11714: F, t11967: F, t11972: F, t11980: F, t11989: F, t12007: F, t12010: F, t1671: F, t1675: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t16076, t16078, t16081, t16084, t16087, t16088) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1611::<F>(t1651, t3133, t1045, t3117, t12167, t15905, t11631, t3151, t15907, t3057, t380, t3088, t370);
        let (t16089, t16091, t16094, t16095, t16096, t16098, t16103) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1612::<F>(t16087, t16088, t4757, t906, t3092, t380, t994, t606, t999, t4578, t905, t1045);
        let (t16104, t16114) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1613::<F>(t15691, t16103, t11774, t11917, t11924, t11938, t11952, t11954, t11956, t11965, t16078, t16081, t16084, t16089, t16091, t16095, t16098, t3115);
        let (t16123, t16128, t16136) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1614::<F>(t3169, t4820, t1015, t13312, t1012, t16096, t4573, t11703, t3188, t4817, t1011, t11268, t11714, t11967, t11972, t11980, t11989, t12007, t12010, t16095, t1671, t1675);
    (t16076, t16078, t16084, t16087, t16091, t16094, t16098, t16104, t16114, t16123, t16128, t16136)
}
