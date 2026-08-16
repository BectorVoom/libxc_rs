//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1330/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1330(t108282: f64, t108494: f64, t108496: f64, t108498: f64, t108662: f64, t22975: f64, t27909: f64, t30057: f64, t543: f64, t6843: f64, t6896: f64, t7279: f64, t7295: f64, t7301: f64, t7910: f64, t7917: f64, t7921: f64, t94917: f64, t94931: f64, t98314: f64, t98333: f64, t98338: f64, t98372: f64) -> f64 {
    let t114740 = -0.51405703062096148814e-2_f64 * t98314 + 0.43368140941025997312e-1_f64 * t108494 - 0.77108554593144223218e-1_f64 * t108496 - 0.58544643236296698113e-1_f64 * t108498 - 0.13010442282307799193e1_f64 * t7917 * t30057 - 0.10281140612419229762e0_f64 * t98333 - 0.10281140612419229763e-1_f64 * t98338 + 0.13010442282307799193e1_f64 * t7295 * t7301 * t7910 * t6843 * t543 - 0.39512695097613069591e1_f64 * t7279 * t22975 + 0.39512695097613069591e1_f64 * t27909 * t6896 - 0.39029762157531132076e-1_f64 * t98372 + t94917 - t94931 + 0.32927245914677557992e-1_f64 * t108662 + 0.26020884564615598386e1_f64 * t108282 * t7921;
    t114740
}
