//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta589 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2012;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2013;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta589<F: Float>(t94483: F, t64: F, t9990: F, t2482: F, t596: F, t7262: F, t4021: F, t25981: F, t27: F, t550: F, t7021: F, t25273: F, t540: F, t1372: F, t2019: F, t9951: F, t2018: F, t9646: F, t9723: F, t26014: F, t2689: F, t3994: F, t7028: F, t9845: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t94484, t94491, t94497, t94498, t94508, t94513, t94519) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2012::<F>(t94483, t64, t9990, t2482, t596, t7262, t4021, t25981, t27, t550, t7021, t25273, t540);
        let (t94520, t94523, t94526, t94527, t94537) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2013::<F>(t1372, t94519, t2019, t9951, t2018, t9646, t9723, t26014, t2689, t3994, t7028, t9845);
    (t94484, t94491, t94497, t94498, t94508, t94513, t94519, t94520, t94523, t94526, t94527, t94537)
}
