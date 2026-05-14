//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1144/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1144<F: Float>(t130901: F, t130907: F, t130951: F, t130961: F, t130975: F, t130984: F, t131000: F, t131005: F, t131018: F, t131037: F, t131045: F, t131064: F, t131080: F, t131092: F, t131103: F, t131115: F) -> (F,) {
    let t131119 = t130901 + t130907 + t130951 + t130961 + t130975 + t130984 + t131000 + t131005 + t131018 + t131037 + t131045 + t131064 + t131080 + t131092 + 2.0 * t131103 + t131115;
    (t131119,)
}
