//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 482/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk482(t1352: f64, t987: f64, t203: f64, t2754: f64, t599: f64, t158: f64, t2796: f64, t501: f64, t1381: f64, t997: f64, t2876: f64, t540: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7974 = t987 * t1352;
    let t7980 = t203 * t2754;
    let t7995 = t599 * t2754;
    let t8025 = t158 * t2754;
    let t8042 = t2796 * t501;
    let t8045 = t997 * t1381;
    let t8063 = t2876 * t540;
    (t7974, t7980, t7995, t8025, t8042, t8045, t8063)
}
