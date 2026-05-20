//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta525 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1874;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1875;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta525<F: Float>(t27879: F, t27907: F, t27984: F, t28017: F, t532: F, t1450: F, t2014: F, t1513: F, t25823: F, t665: F, t25826: F, t4287: F, t6998: F, t114: F, t25822: F, t25824: F) -> (F, F, F, F, F, F, F) {
        let (t28019, t28020, t28021, t28022, t28034, t28036, t28037, t28039) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1874::<F>(t27879, t27907, t27984, t28017, t532, t1450, t2014, t1513, t25823, t665, t25826, t4287, t6998);
        let t28042 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1875::<F>(t114, t25822, t25824, t28034, t28037, t28039);
    (t28019, t28020, t28021, t28022, t28034, t28036, t28042)
}
