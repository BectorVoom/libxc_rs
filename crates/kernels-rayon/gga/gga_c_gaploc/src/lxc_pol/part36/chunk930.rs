//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 930/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk930(t40588: f64, t40591: f64, t40596: f64, t40599: f64, t40602: f64, t13194: f64, t29439: f64, t32357: f64, t5539: f64, t9647: f64, t32436: f64, t42920: f64, t701: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t42980 = 0.1922631557535556071e-2_f64 * t40588;
    let t42981 = 0.4486140300916297499e-2_f64 * t40591;
    let t42982 = 0.7690526230142224284e-2_f64 * t40596;
    let t42983 = 0.3845263115071112142e-2_f64 * t40599;
    let t42984 = 0.1281754371690370714e-2_f64 * t40602;
    let t42985 = t29439 * t13194;
    let t42986 = 0.12817543716903707139e-2_f64 * t42985;
    let t42988 = t9647 * t5539 * t32357;
    let t42989 = 0.12817543716903707139e-2_f64 * t42988;
    let t42991 = t9647 * t5539 * t32436;
    let t42992 = 0.12817543716903707139e-2_f64 * t42991;
    let t42993 = t42920 * t701;
    (t42980, t42981, t42982, t42983, t42984, t42986, t42989, t42992, t42993)
}
