//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta863 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2754;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2755;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta863(t13725: f64, t1904: f64, t2439: f64, t1364: f64, t22441: f64, t786: f64, t22446: f64, t2435: f64, t14079: f64, t14100: f64, t3895: f64, t6919: f64, t10175: f64, t22399: f64, t13734: f64, t689: f64, t2453: f64, t3908: f64, t6889: f64, t22398: f64, t2470: f64, t3915: f64, t22452: f64, t9680: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t73593, t73598, t73623, t73627, t73641) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2754(t13725, t1904, t2439, t1364, t22441, t786, t22446, t2435, t14079, t14100, t3895, t6919);
        let (t73647, t73652, t73656, t73662, t73666) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2755(t10175, t22399, t13734, t1904, t689, t2453, t3908, t6889, t22398, t2470, t3915, t22452, t9680);
    (t73593, t73598, t73623, t73627, t73641, t73647, t73652, t73656, t73662, t73666)
}
