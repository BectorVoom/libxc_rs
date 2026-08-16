//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta508 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2130;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta508<F: Float>(t15140: F, t16012: F, t15780: F, t4900: F, t3117: F, t3133: F, t357: F, t4893: F, t3059: F, t4781: F, t1011: F, t11927: F, t11933: F, t15996: F, t15997: F, t16000: F, t16003: F, t16006: F, t16009: F, t4899: F, t4907: F, t4912: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t16013, t16016, t16017, t16020, t16021, t16022, t16025, t16026, t16027, t16034) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2130::<F>(t15140, t16012, t15780, t4900, t3117, t3133, t357, t4893, t3059, t4781, t1011, t11927, t11933, t15996, t15997, t16000, t16003, t16006, t16009, t4899, t4907, t4912);
    (t16013, t16016, t16017, t16020, t16021, t16022, t16025, t16026, t16027, t16034)
}
