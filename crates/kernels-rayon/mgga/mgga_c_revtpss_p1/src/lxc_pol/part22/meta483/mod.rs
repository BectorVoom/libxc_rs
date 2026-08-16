//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta483 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2198;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2199;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta483(t15993: f64, t4574: f64, t1011: f64, t15145: f64, t4915: f64, t15149: f64, t15154: f64, t4919: f64, t15130: f64, t15135: f64, t1012: f64, t11821: f64, t15140: f64, t15780: f64, t4900: f64, t3117: f64, t3133: f64, t357: f64, t4893: f64, t3059: f64, t4781: f64, t11927: f64, t11933: f64, t4899: f64, t4907: f64, t4912: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15994, t15996, t15997, t16000, t16003, t16006, t16009, t16012) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2198(t15993, t4574, t1011, t15145, t4915, t15149, t15154, t4919, t15130, t15135, t1012, t11821);
        let (t16016, t16017, t16020, t16021, t16022, t16025, t16026, t16027, t16034) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2199(t15140, t16012, t15780, t4900, t3117, t3133, t357, t4893, t3059, t4781, t1011, t11927, t11933, t15996, t15997, t16000, t16003, t16006, t16009, t4899, t4907, t4912);
    (t15994, t15996, t16012, t16016, t16017, t16020, t16021, t16022, t16025, t16026, t16027, t16034)
}
