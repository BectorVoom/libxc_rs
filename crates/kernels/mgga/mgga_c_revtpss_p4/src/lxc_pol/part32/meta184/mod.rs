//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta184 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk838;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk839;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk840;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta184<F: Float>(t4574: F, t4919: F, t140: F, t1655: F, t1011: F, t1656: F, t3115: F, t3234: F, t3241: F, t3245: F, t4887: F, t4892: F, t4896: F, t4899: F, t4902: F, t4907: F, t4912: F, t4916: F, t1063: F, t1671: F, t3082: F, t3086: F, t3091: F, t3169: F, t375: F, t4783: F, t4788: F, t4792: F, t4794: F, t4798: F, t4803: F, t4808: F, t4848: F, t4883: F, t225: F, t385: F, t1678: F, t342: F, t1695: F, t999: F, t1079: F, t1096: F, t3269: F, t1086: F, t1647: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t4924, t4925, t4928) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk838::<F>(t4574, t4919, t140, t1655, t1011, t1656, t3115, t3234, t3241, t3245, t4887, t4892, t4896, t4899, t4902, t4907, t4912, t4916);
        let t4930 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk839::<F>(t1063, t1671, t3082, t3086, t3091, t3169, t375, t4783, t4788, t4792, t4794, t4798, t4803, t4808, t4848, t4883, t4928);
        let (t4932, t4935, t4940, t4941, t4946, t4947, t4954) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk840::<F>(t225, t385, t4930, t1678, t342, t1695, t999, t1079, t1096, t3269, t1086, t1647);
    (t4924, t4925, t4930, t4932, t4935, t4940, t4941, t4946, t4947, t4954)
}
