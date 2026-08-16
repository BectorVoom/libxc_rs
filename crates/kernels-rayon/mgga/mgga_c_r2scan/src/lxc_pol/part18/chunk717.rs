//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 717/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk717(t5632: f64, t664: f64, t2006: f64, t206: f64, t2008: f64, t1966: f64, t188: f64, t650: f64, t5771: f64, t621: f64, t226: f64, t5317: f64) -> (f64, f64, f64, f64, f64) {
    let t5782 = t5632 * t664;
    let t5785 = t2006 * t206;
    let t5786 = t2008 * t664;
    let t5787 = t5786 * t1966;
    let t5790 = t650 * t188;
    let t5791 = t5771 * t621;
    let t5793 = 18.0_f64 * t5790 * t5791;
    let t5794 = t226 * t5317;
    (t5782, t5785, t5787, t5793, t5794)
}
