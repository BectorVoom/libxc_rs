//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta635 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2203;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta635<F: Float>(t13435: F, t7741: F, t2322: F, t28042: F, t13440: F, t5523: F, t25191: F, t7898: F, t1937: F, t49686: F, t75667: F, t13426: F, t6993: F) -> (F, F, F, F, F, F, F, F) {
        let (t101534, t101536, t101538, t101540, t101546, t101548, t101550, t101552) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2203::<F>(t13435, t7741, t2322, t28042, t13440, t5523, t25191, t7898, t1937, t49686, t75667, t13426, t6993);
    (t101534, t101536, t101538, t101540, t101546, t101548, t101550, t101552)
}
