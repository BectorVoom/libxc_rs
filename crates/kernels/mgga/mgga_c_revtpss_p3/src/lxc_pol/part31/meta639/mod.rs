//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta639 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2095;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2096;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta639<F: Float>(t28019: F, t531: F, t1513: F, t94975: F, t28036: F, t94978: F, t25823: F, t4287: F, t1913: F, t7337: F, t116: F, t28042: F, t28283: F, t571: F, t28234: F, t575: F, t1455: F, t7956: F, t1464: F, t7939: F, t2037: F, t5808: F, t1921: F, t7318: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t101417, t101451, t101454, t101456, t101563, t101622) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2095::<F>(t28019, t531, t1513, t94975, t28036, t94978, t25823, t4287, t1913, t7337, t116, t28042);
        let (t101656, t101658, t101661, t101668, t101670, t101672) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2096::<F>(t28283, t571, t28234, t575, t1455, t7956, t1464, t7939, t2037, t5808, t1921, t7318);
    (t101417, t101451, t101454, t101456, t101563, t101622, t101656, t101658, t101661, t101668, t101670, t101672)
}
