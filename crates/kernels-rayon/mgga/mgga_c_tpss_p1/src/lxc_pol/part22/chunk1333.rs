//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1333/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1333(t12898: f64, t19476: f64, t12970: f64, t18454: f64, t12974: f64, t13009: f64, t12883: f64, t18444: f64, t339: f64, t4419: f64, t790: f64, t1246: f64, t136: f64, t1693: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t65580 = t19476 * t12898;
    let t65582 = t18454 * t12970;
    let t65584 = t18454 * t12974;
    let t65586 = t19476 * t13009;
    let t65588 = t18454 * t12883;
    let t65592 = t339 * t18444 * t790 * t4419;
    let t65595 = t1693 * t1246 * t136;
    (t65580, t65582, t65584, t65586, t65588, t65592, t65595)
}
