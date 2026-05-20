//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta426 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1604;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1605;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1606;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1607;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta426<F: Float>(t15964: F, t3092: F, t11659: F, t3154: F, t1592: F, t357: F, t11710: F, t4782: F, t3091: F, t1014: F, t140: F, t4579: F, t1011: F, t11672: F, t11675: F, t11881: F, t11886: F, t12004: F, t15952: F, t15959: F, t1675: F, t3127: F, t4783: F, t4892: F, t4899: F, t3252: F, t4574: F, t15145: F, t4915: F, t15149: F, t15154: F, t4919: F, t15130: F, t15135: F, t1012: F, t11821: F, t15140: F, t15780: F, t4900: F, t3117: F, t3133: F, t4893: F, t3059: F, t4781: F, t11927: F, t11933: F, t4907: F, t4912: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t15965, t15970, t15975, t15984, t15986, t15988) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1604::<F>(t15964, t3092, t11659, t3154, t1592, t357, t11710, t4782, t3091, t1014, t140, t4579);
        let t15991 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1605::<F>(t1011, t15988, t11672, t11675, t11881, t11886, t12004, t15952, t15959, t15965, t15970, t15975, t15986, t1675, t3091, t3127, t4783, t4892, t4899);
        let (t15996, t15997, t16000, t16003, t16006, t16009, t16012) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1606::<F>(t140, t3252, t4574, t1011, t15145, t4915, t15149, t15154, t4919, t15130, t15135, t1012, t11821);
        let (t16017, t16022, t16027, t16034) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1607::<F>(t15140, t16012, t15780, t4900, t3117, t3133, t357, t4893, t3059, t4781, t1011, t11927, t11933, t15996, t15997, t16000, t16003, t16006, t16009, t4899, t4907, t4912);
    (t15965, t15970, t15975, t15984, t15991, t16017, t16022, t16027, t16034)
}
