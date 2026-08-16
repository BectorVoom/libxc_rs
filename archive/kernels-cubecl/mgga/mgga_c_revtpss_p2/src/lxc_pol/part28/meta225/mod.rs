//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta225 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1061;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1062;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta225<F: Float>(t3357: F, t3358: F, t5044: F, t5049: F, t5054: F, t5058: F, t422: F, t1130: F, t1719: F, t1151: F, t1733: F, t3379: F, t1149: F, t3384: F, t1723: F, t3390: F, t1134: F, t3394: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t5060, t5062, t5063, t5065, t5067) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1061::<F>(t3357, t3358, t5044, t5049, t5054, t5058, t422, t1130, t1719, t1151, t1733, t3379);
        let (t5068, t5070, t5071, t5072, t5079) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1062::<F>(t1149, t1733, t3384, t1723, t3390, t1134, t3358, t3394, t5044, t5049, t5054, t5058);
    (t5060, t5062, t5063, t5065, t5067, t5068, t5070, t5071, t5072, t5079)
}
