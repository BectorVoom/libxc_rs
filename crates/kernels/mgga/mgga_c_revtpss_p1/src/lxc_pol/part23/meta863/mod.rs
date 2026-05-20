//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta863 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2754;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2755;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta863<F: Float>(t13725: F, t1904: F, t2439: F, t1364: F, t22441: F, t786: F, t22446: F, t2435: F, t14079: F, t14100: F, t3895: F, t6919: F, t10175: F, t22399: F, t13734: F, t689: F, t2453: F, t3908: F, t6889: F, t22398: F, t2470: F, t3915: F, t22452: F, t9680: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t73593, t73598, t73623, t73627, t73641) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2754::<F>(t13725, t1904, t2439, t1364, t22441, t786, t22446, t2435, t14079, t14100, t3895, t6919);
        let (t73647, t73652, t73656, t73662, t73666) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2755::<F>(t10175, t22399, t13734, t1904, t689, t2453, t3908, t6889, t22398, t2470, t3915, t22452, t9680);
    (t73593, t73598, t73623, t73627, t73641, t73647, t73652, t73656, t73662, t73666)
}
