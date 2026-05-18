//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 498/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk498<F: Float>(t2457: F, t3907: F, t3906: F, t1420: F, t786: F, t1364: F, t1426: F, t556: F, t1444: F, t676: F, t123: F, t1363: F, t2470: F) -> (F, F, F, F, F, F, F, F) {
    let t3908 = t3907 * t2457;
    let t3910 = F::new(0.11565819519348392139e-2) * t3906 * t3908;
    let t3911 = t786 * t1420;
    let t3912 = t3911 * t1364;
    let t3914 = t556 * t1426;
    let t3915 = t786 * t3914;
    let t3916 = t676 * t1444;
    let t3917 = t123 * t3916;
    let t3918 = t3915 * t3917;
    let t3920 = t1363 * t2470;
    (t3908, t3910, t3912, t3915, t3916, t3917, t3918, t3920)
}
