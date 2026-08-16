//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 720/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk720(t206: f64, t687: f64, t5507: f64, t664: f64, t673: f64, t621: f64, t631: f64, t5771: f64, t225: f64, t5270: f64, t717: f64, t1813: f64, t1966: f64, t2017: f64, t2030: f64, t207: f64, t5549: f64, t5589: f64, t5798: f64, t5801: f64, t5812: f64, t5815: f64, t5818: f64, t5821: f64, t674: f64, t686: f64, t690: f64, t705: f64) -> (f64, f64, f64) {
    let t5822 = t687 * t206;
    let t5823 = t5507 * t664;
    let t5829 = t673 * t664;
    let t5832 = t631 * t621;
    let t5834 = 0.12822e1_f64 * t5832 * t5771;
    let t5836 = t717 * t5270 * t225;
    let t5841 = -0.35089341735807877242e1_f64 * t705 * t5798 + 0.57791679765211885293e1_f64 * t5801 * t1813 + 0.96491876992155210402e2_f64 * t687 * t2017 * t1966 + 0.32163958997385070134e2_f64 * t687 * t690 * t5549 + t5812 + t5815 - t5818 + t5821 + 18.0_f64 * t5822 * t5823 - 6.0_f64 * t674 * t2030 * t664 - 0.123288e1_f64 * t5829 * t5507 + t5834 + 0.3903689268108626343e0_f64 * t5836 + 0.123288e1_f64 * t686 * t5589 * t207;
    (t5834, t5836, t5841)
}
