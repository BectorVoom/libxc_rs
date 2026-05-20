//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta637 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2206;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2207;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta637<F: Float>(t1459: F, t28277: F, t28280: F, t5795: F, t7331: F, t28268: F, t116: F, t28042: F, t572: F, t670: F, t2371: F, t28276: F, t4158: F, t7950: F, t18190: F, t2042: F, t28271: F, t7334: F, t1518: F, t95137: F, t26123: F, t4292: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t101613, t101617, t101619, t101621, t101625, t101628) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2206::<F>(t1459, t28277, t28280, t5795, t7331, t28268, t116, t28042, t572, t670, t2371, t28276);
        let (t101632, t101634, t101640, t101642, t101645, t101648) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2207::<F>(t4158, t7950, t18190, t2042, t1459, t28271, t5795, t7334, t1518, t572, t95137, t26123, t4292);
    (t101613, t101617, t101619, t101621, t101625, t101628, t101632, t101634, t101640, t101642, t101645, t101648)
}
