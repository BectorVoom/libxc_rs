//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2091/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2091<F: Float>(t25901: F, t97802: F, t1364: F, t27961: F, t786: F, t2453: F, t3908: F, t7911: F, t136: F, t2457: F, t7920: F, t94589: F) -> (F, F, F, F, F) {
    let t97804 = F::cast_from(0.14456046980341999104e-1_f64) * t97802 * t25901;
    let t97808 = F::cast_from(0.19514881078765566038e-1_f64) * t786 * t27961 * t1364;
    let t97810 = t2453 * t7911 * t3908;
    let t97814 = t7920 * t136 * t2457;
    let t97815 = t94589 * t97814;
    (t97804, t97808, t97810, t97814, t97815)
}
