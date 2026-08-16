//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta508 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2130;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta508(t15140: f64, t16012: f64, t15780: f64, t4900: f64, t3117: f64, t3133: f64, t357: f64, t4893: f64, t3059: f64, t4781: f64, t1011: f64, t11927: f64, t11933: f64, t15996: f64, t15997: f64, t16000: f64, t16003: f64, t16006: f64, t16009: f64, t4899: f64, t4907: f64, t4912: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16013, t16016, t16017, t16020, t16021, t16022, t16025, t16026, t16027, t16034) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2130(t15140, t16012, t15780, t4900, t3117, t3133, t357, t4893, t3059, t4781, t1011, t11927, t11933, t15996, t15997, t16000, t16003, t16006, t16009, t4899, t4907, t4912);
    (t16013, t16016, t16017, t16020, t16021, t16022, t16025, t16026, t16027, t16034)
}
