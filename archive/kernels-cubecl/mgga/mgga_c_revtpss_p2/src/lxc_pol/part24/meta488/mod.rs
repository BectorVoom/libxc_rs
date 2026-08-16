//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta488 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1481;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1482;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta488<F: Float>(t22212: F, t2516: F, t6922: F, t9593: F, t22185: F, t2619: F, t22404: F, t3920: F, t13725: F, t1904: F, t2439: F, t22446: F, t2435: F, t3895: F, t6919: F, t2453: F, t3908: F, t6889: F, t22398: F, t2470: F, t3915: F, t22452: F, t9680: F, t22409: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t73481, t73499, t73515, t73587, t73593, t73623) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1481::<F>(t22212, t2516, t6922, t9593, t22185, t2619, t22404, t3920, t13725, t1904, t2439, t22446, t2435);
        let (t73641, t73656, t73662, t73666, t73673) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1482::<F>(t2439, t3895, t6919, t2453, t3908, t6889, t22398, t2470, t3915, t22452, t9680, t22409, t2435);
    (t73481, t73499, t73515, t73587, t73593, t73623, t73641, t73656, t73662, t73666, t73673)
}
