//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1021/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1021(t10782: f64, t701: f64, t1445: f64, t3504: f64, t5782: f64, t8483: f64, t935: f64, t2087: f64, t2530: f64, t3009: f64, t10721: f64, t10006: f64, t10993: f64, t10995: f64, t10996: f64, t11001: f64, t11006: f64, t11010: f64, t11011: f64, t11012: f64, t11015: f64, t11018: f64, t1998: f64, t2009: f64, t780: f64, t807: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11019 = t10782 * t701;
    let t11020 = t1445 * t11019;
    let t11024 = 0.69017266717057349418e1_f64 * t5782 * t3504;
    let t11025 = t8483 * t935;
    let t11026 = t1445 * t11025;
    let t11028 = 0.69017266717057349418e1_f64 * t2087 * t11026;
    let t11029 = t3009 * t2530;
    let t11030 = t1445 * t11029;
    let t11032 = 0.69017266717057349418e1_f64 * t2087 * t11030;
    let t11033 = t1445 * t10721;
    let t11036 = -t10993 + t10995 - 0.35750489951850426669e0_f64 * t10996 * t2009 + 0.35750489951850426669e0_f64 * t780 * t11001 - 0.69017266717057349418e1_f64 * t2087 * t11006 + t11010 + t10006 - t11011 + t11012 + t11015 - t11018 - 0.23005755572352449806e1_f64 * t1998 * t11020 - t11024 - t11028 - t11032 + 0.23005755572352449806e1_f64 * t807 * t11033;
    (t11019, t11020, t11025, t11026, t11029, t11030, t11033, t11036)
}
