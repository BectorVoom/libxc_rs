//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1330/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1330<F: Float>(t108282: F, t108494: F, t108496: F, t108498: F, t108662: F, t22975: F, t27909: F, t30057: F, t543: F, t6843: F, t6896: F, t7279: F, t7295: F, t7301: F, t7910: F, t7917: F, t7921: F, t94917: F, t94931: F, t98314: F, t98333: F, t98338: F, t98372: F) -> F {
    let t114740 = -F::cast_from(0.51405703062096148814e-2_f64) * t98314 + F::cast_from(0.43368140941025997312e-1_f64) * t108494 - F::cast_from(0.77108554593144223218e-1_f64) * t108496 - F::cast_from(0.58544643236296698113e-1_f64) * t108498 - F::cast_from(0.13010442282307799193e1_f64) * t7917 * t30057 - F::cast_from(0.10281140612419229762e0_f64) * t98333 - F::cast_from(0.10281140612419229763e-1_f64) * t98338 + F::cast_from(0.13010442282307799193e1_f64) * t7295 * t7301 * t7910 * t6843 * t543 - F::cast_from(0.39512695097613069591e1_f64) * t7279 * t22975 + F::cast_from(0.39512695097613069591e1_f64) * t27909 * t6896 - F::cast_from(0.39029762157531132076e-1_f64) * t98372 + t94917 - t94931 + F::cast_from(0.32927245914677557992e-1_f64) * t108662 + F::cast_from(0.26020884564615598386e1_f64) * t108282 * t7921;
    t114740
}
