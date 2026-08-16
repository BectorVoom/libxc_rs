//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1334/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1334(t12851: f64, t215: f64, t65595: f64, t19468: f64, t19470: f64, t5543: f64, t12825: f64, t18454: f64, t236: f64, t339: f64, t60698: f64, t12894: f64) -> (f64, f64, f64, f64) {
    let t65597 = t65595 * t215 * t12851;
    let t65600 = t5543 * t19468 * t19470;
    let t65604 = t18454 * t12825;
    let t65607 = t339 * t60698 * t236;
    let t65608 = t65607 * t12894;
    (t65597, t65600, t65604, t65608)
}
