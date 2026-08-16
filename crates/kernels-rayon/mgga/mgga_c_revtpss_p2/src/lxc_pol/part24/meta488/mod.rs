//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta488 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1481;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1482;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta488(t22212: f64, t2516: f64, t6922: f64, t9593: f64, t22185: f64, t2619: f64, t22404: f64, t3920: f64, t13725: f64, t1904: f64, t2439: f64, t22446: f64, t2435: f64, t3895: f64, t6919: f64, t2453: f64, t3908: f64, t6889: f64, t22398: f64, t2470: f64, t3915: f64, t22452: f64, t9680: f64, t22409: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t73481, t73499, t73515, t73587, t73593, t73623) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1481(t22212, t2516, t6922, t9593, t22185, t2619, t22404, t3920, t13725, t1904, t2439, t22446, t2435);
        let (t73641, t73656, t73662, t73666, t73673) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1482(t2439, t3895, t6919, t2453, t3908, t6889, t22398, t2470, t3915, t22452, t9680, t22409, t2435);
    (t73481, t73499, t73515, t73587, t73593, t73623, t73641, t73656, t73662, t73666, t73673)
}
