//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta426 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1604;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1605;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1606;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1607;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta426(t15964: f64, t3092: f64, t11659: f64, t3154: f64, t1592: f64, t357: f64, t11710: f64, t4782: f64, t3091: f64, t1014: f64, t140: f64, t4579: f64, t1011: f64, t11672: f64, t11675: f64, t11881: f64, t11886: f64, t12004: f64, t15952: f64, t15959: f64, t1675: f64, t3127: f64, t4783: f64, t4892: f64, t4899: f64, t3252: f64, t4574: f64, t15145: f64, t4915: f64, t15149: f64, t15154: f64, t4919: f64, t15130: f64, t15135: f64, t1012: f64, t11821: f64, t15140: f64, t15780: f64, t4900: f64, t3117: f64, t3133: f64, t4893: f64, t3059: f64, t4781: f64, t11927: f64, t11933: f64, t4907: f64, t4912: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15965, t15970, t15975, t15984, t15986, t15988) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1604(t15964, t3092, t11659, t3154, t1592, t357, t11710, t4782, t3091, t1014, t140, t4579);
        let t15991 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1605(t1011, t15988, t11672, t11675, t11881, t11886, t12004, t15952, t15959, t15965, t15970, t15975, t15986, t1675, t3091, t3127, t4783, t4892, t4899);
        let (t15996, t15997, t16000, t16003, t16006, t16009, t16012) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1606(t140, t3252, t4574, t1011, t15145, t4915, t15149, t15154, t4919, t15130, t15135, t1012, t11821);
        let (t16017, t16022, t16027, t16034) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1607(t15140, t16012, t15780, t4900, t3117, t3133, t357, t4893, t3059, t4781, t1011, t11927, t11933, t15996, t15997, t16000, t16003, t16006, t16009, t4899, t4907, t4912);
    (t15965, t15970, t15975, t15984, t15991, t16017, t16022, t16027, t16034)
}
