//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta526 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1898;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1899;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta526<F: Float>(t25904: F, t27989: F, t25899: F, t2022: F, t5774: F, t7296: F, t1955: F, t5710: F, t27960: F, t545: F, t2028: F, t1904: F, t2027: F, t2030: F, t26062: F, t26065: F, t26067: F, t26071: F, t26073: F, t26084: F, t27987: F, t5728: F, t7279: F, t7292: F, t7295: F, t7308: F, t7917: F, t7930: F, t27879: F, t27907: F, t27984: F, t532: F, t1450: F, t2014: F, t1931: F, t670: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t27990, t27992, t28002, t28003, t28008, t28011, t28012, t28017) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1898::<F>(t25904, t27989, t25899, t2022, t5774, t7296, t1955, t5710, t27960, t545, t2028, t1904, t2027, t2030, t26062, t26065, t26067, t26071, t26073, t26084, t27987, t5728, t7279, t7292, t7295, t7308, t7917, t7930);
        let (t28019, t28020, t28021, t28022, t28025) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1899::<F>(t27879, t27907, t27984, t28017, t532, t1450, t2014, t1931, t670);
    (t27990, t27992, t28002, t28003, t28008, t28011, t28012, t28019, t28020, t28021, t28022, t28025)
}
