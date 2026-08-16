//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 608/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk608(t11016: f64, t3025: f64, t10782: f64, t701: f64, t1445: f64, t3504: f64, t5782: f64, t8483: f64, t935: f64, t2087: f64, t2530: f64, t3009: f64) -> (f64, f64, f64, f64, f64) {
    let t11018 = 0.7150097990370085334e0_f64 * t3025 * t11016;
    let t11019 = t10782 * t701;
    let t11020 = t1445 * t11019;
    let t11024 = 0.69017266717057349418e1_f64 * t5782 * t3504;
    let t11025 = t8483 * t935;
    let t11026 = t1445 * t11025;
    let t11028 = 0.69017266717057349418e1_f64 * t2087 * t11026;
    let t11029 = t3009 * t2530;
    (t11018, t11020, t11024, t11028, t11029)
}
