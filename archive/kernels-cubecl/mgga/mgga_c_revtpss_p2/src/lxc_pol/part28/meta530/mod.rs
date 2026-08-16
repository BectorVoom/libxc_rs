//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta530 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1968;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1969;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1970;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1971;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta530<F: Float>(t27879: F, t27907: F, t27984: F, t28017: F, t532: F, t1450: F, t2014: F, t1931: F, t670: F, t116: F, t7724: F, t114: F, t1513: F, t25823: F, t665: F, t25826: F, t4287: F, t6998: F, t25822: F, t25824: F, t508: F, t651: F, t118: F, t1519: F, t25805: F, t27145: F, t27152: F, t27156: F, t27830: F, t27834: F, t27835: F, t4254: F, t4257: F, t4293: F, t4297: F, t671: F, t6985: F, t7746: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t28019, t28020, t28021, t28022, t28025) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1968::<F>(t27879, t27907, t27984, t28017, t532, t1450, t2014, t1931, t670);
        let t28030 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1969::<F>(t116, t7724);
        let (t28036, t28042) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1970::<F>(t114, t1513, t25823, t665, t25826, t4287, t6998, t25822, t25824);
        let (t28043, t28046) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1971::<F>(t28042, t508, t651, t118, t1519, t25805, t27145, t27152, t27156, t27830, t27834, t27835, t28022, t28025, t28030, t4254, t4257, t4293, t4297, t671, t6985, t7746);
    (t28019, t28020, t28021, t28025, t28030, t28036, t28042, t28043, t28046)
}
